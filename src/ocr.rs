//! OCR module - Text extraction from screen regions using Tesseract.
use anyhow::{anyhow, Result};
use image::codecs::png::PngEncoder;
use image::{ColorType, DynamicImage, ImageBuffer, ImageEncoder, Rgba};
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn rgba_image_from_raw(data: &[u8], width: u32, height: u32) -> Result<DynamicImage> {
    let expected = (width as usize)
        .checked_mul(height as usize)
        .and_then(|v| v.checked_mul(4))
        .ok_or_else(|| anyhow!("Invalid image dimensions"))?;
    if data.len() != expected {
        return Err(anyhow!(
            "Invalid image data length: expected {}, got {}",
            expected,
            data.len()
        ));
    }
    let buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_raw(width, height, data.to_vec())
            .ok_or_else(|| anyhow!("Failed to build image buffer"))?;
    Ok(DynamicImage::ImageRgba8(buffer))
}

fn crop_region(
    image: &DynamicImage,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    frame_width: u32,
    frame_height: u32,
) -> Result<DynamicImage> {
    if x < 0 || y < 0 {
        return Err(anyhow!("Region out of bounds: negative origin"));
    }
    let x = x as u32;
    let y = y as u32;
    if width == 0 || height == 0 {
        return Err(anyhow!("Region out of bounds: zero size"));
    }
    if x + width > frame_width || y + height > frame_height {
        return Err(anyhow!("Region out of bounds: exceeds frame"));
    }
    Ok(image.crop_imm(x, y, width, height))
}

fn encode_png(image: &DynamicImage) -> Result<Vec<u8>> {
    let mut buf = Vec::new();
    let rgba = image.to_rgba8();
    let encoder = PngEncoder::new(&mut buf);
    encoder.write_image(rgba.as_raw(), rgba.width(), rgba.height(), ColorType::Rgba8.into())?;
    Ok(buf)
}

fn find_tesseract() -> Result<PathBuf> {
    if let Ok(path) = std::env::var("TESSERACT_PATH") {
        let pb = PathBuf::from(path);
        if pb.is_file() {
            return Ok(pb);
        }
    }

    let default_path = PathBuf::from(r"C:\Program Files\Tesseract-OCR\tesseract.exe");
    if default_path.is_file() {
        return Ok(default_path);
    }

    Err(anyhow!(
        "Tesseract not found. Set TESSERACT_PATH to tesseract.exe"
    ))
}

fn ocr_image_bytes(png_bytes: &[u8]) -> Result<String> {
    let mut path = PathBuf::from(std::env::temp_dir());
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis())
        .unwrap_or(0);
    path.push(format!("agent_core_ocr_{}.png", stamp));
    fs::write(&path, png_bytes)?;

    let tesseract_path = find_tesseract()?;
    let output = Command::new(tesseract_path)
        .arg(&path)
        .arg("stdout")
        .arg("-l")
        .arg("eng")
        .output()
        .map_err(|e| anyhow!("Failed to run tesseract: {}", e))?;

    let _ = fs::remove_file(&path);

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(anyhow!("Tesseract error: {}", err));
    }

    let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(text)
}

/// OCR a single region from a full RGBA frame.
pub fn ocr_region_impl(
    frame_data: &[u8],
    frame_width: u32,
    frame_height: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> Result<String> {
    let image = rgba_image_from_raw(frame_data, frame_width, frame_height)?;
    let cropped = crop_region(&image, x, y, width, height, frame_width, frame_height)?;
    let png_bytes = encode_png(&cropped)?;
    ocr_image_bytes(&png_bytes)
}

/// OCR multiple regions from a full RGBA frame.
pub fn ocr_regions_impl(
    frame_data: &[u8],
    frame_width: u32,
    frame_height: u32,
    regions: &[(i32, i32, u32, u32)],
) -> Result<Vec<String>> {
    let image = rgba_image_from_raw(frame_data, frame_width, frame_height)?;
    regions
        .par_iter()
        .map(|(x, y, w, h)| {
            let cropped = crop_region(&image, *x, *y, *w, *h, frame_width, frame_height)?;
            let png_bytes = encode_png(&cropped)?;
            ocr_image_bytes(&png_bytes)
        })
        .collect()
}
