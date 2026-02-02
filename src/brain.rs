//! Brain module - Moondream vision-language model for understanding screens
//!
//! Uses Candle to run Moondream2 locally for visual understanding.

use anyhow::{Error, Result};
use candle_core::{DType, Device, Module, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::models::moondream::{Config, Model};
use hf_hub::{api::sync::Api, Repo, RepoType};
use image::DynamicImage;
use tokenizers::Tokenizer;

pub struct Brain {
    model: Model,
    tokenizer: Tokenizer,
    device: Device,
}

impl Brain {
    /// Load the Moondream model (downloads on first run)
    pub fn new() -> Result<Self> {
        println!("Loading Moondream model (this might take a minute on first run)...");

        let device = Device::Cpu;

        // Download from HuggingFace
        let api = Api::new()?;
        let repo = api.repo(Repo::new(
            "vikhyatk/moondream2".to_string(),
            RepoType::Model,
        ));

        let model_file = repo.get("model.safetensors")?;
        let config_file = repo.get("config.json")?;
        let tokenizer_file = repo.get("tokenizer.json")?;

        // Load config
        let config: Config = serde_json::from_str(&std::fs::read_to_string(&config_file)?)?;

        // Load tokenizer
        let tokenizer = Tokenizer::from_file(&tokenizer_file).map_err(Error::msg)?;

        // Load model weights
        let vb = VarBuilder::from_tensors(
            candle_core::safetensors::load(&model_file, &device)?,
            DType::F32,
            &device,
        );

        let model = Model::new(&config, vb)?;
        println!("Model loaded successfully!");

        Ok(Self {
            model,
            tokenizer,
            device,
        })
    }

    /// Look at an image and answer a question about it
    pub fn see_and_think(&mut self, image: &DynamicImage, prompt: &str) -> Result<String> {
        // 1. Resize image to model's expected size (378x378)
        let image = image.resize_exact(378, 378, image::imageops::FilterType::Triangle);
        let image_tensor = self.image_to_tensor(&image)?;

        // 2. Encode image through vision encoder
        let image_embeds = self.model.vision_encoder().forward(&image_tensor)?;

        // 3. Tokenize prompt
        let prompt_str = format!("\n\nQuestion: {}\n\nAnswer:", prompt);
        let tokens = self.tokenizer.encode(prompt_str.as_str(), true).map_err(Error::msg)?;
        let mut token_ids: Vec<u32> = tokens.get_ids().to_vec();

        // 4. Get special tokens
        let eos_token = match self.tokenizer.get_vocab(true).get("<|endoftext|>") {
            Some(&id) => id,
            None => 50256, // Default GPT-2 EOS
        };
        let bos_token_id = match self.tokenizer.get_vocab(true).get("<|endoftext|>") {
            Some(&id) => id,
            None => 50256,
        };

        // 5. Generation loop
        let mut generated_text = String::new();
        let max_tokens = 100;
        let mut first_pass = true;

        for _ in 0..max_tokens {
            let input = Tensor::new(token_ids.as_slice(), &self.device)?.unsqueeze(0)?;

            // First pass includes image embedding
            let logits = if first_pass {
                let bos = Tensor::new(&[bos_token_id], &self.device)?.unsqueeze(0)?;
                first_pass = false;
                self.model.text_model.forward_with_img(&bos, &input, &image_embeds)?
            } else {
                self.model.text_model.forward(&input)?
            };

            // Get logits for last position
            let logits = logits.squeeze(0)?;
            let logits = logits.get(logits.dim(0)? - 1)?;

            // Greedy decode: pick highest probability token
            let next_token = logits.argmax(0)?.to_scalar::<u32>()?;

            // Check for end of text
            if next_token == eos_token {
                break;
            }

            // Decode and append
            let decoded = self.tokenizer.decode(&[next_token], true).map_err(Error::msg)?;
            generated_text.push_str(&decoded);
            token_ids.push(next_token);

            // Stream output
            print!("{}", decoded);
            use std::io::Write;
            std::io::stdout().flush()?;
        }
        println!();

        Ok(generated_text.trim().to_string())
    }

    /// Convert image to tensor in CHW format normalized to 0-1
    fn image_to_tensor(&self, img: &DynamicImage) -> Result<Tensor> {
        let img = img.to_rgb8();
        let (width, height) = img.dimensions();
        let raw_data: Vec<u8> = img.into_raw();

        let tensor = Tensor::from_vec(raw_data, (height as usize, width as usize, 3), &self.device)?
            .permute((2, 0, 1))? // HWC -> CHW
            .to_dtype(DType::F32)?
            .affine(1.0 / 255.0, 0.0)? // Normalize to 0-1
            .unsqueeze(0)?; // Add batch dimension

        Ok(tensor)
    }
}
