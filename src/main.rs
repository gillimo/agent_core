mod brain;
mod eye;

use anyhow::Result;
use brain::Brain;
use eye::Eye;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    
    println!("Initializing Rust Eyes...");
    println!("1. Waking up the Eye (Screen Capture)...");
    let eye = Eye::new()?;
    
    println!("2. Waking up the Brain (Loading Moondream AI)...");
    let mut brain = Brain::new()?;
    
    println!("--- System Online ---");
    println!("Taking a look at your screen...");

    let start = Instant::now();
    let frame = eye.capture()?;
    println!("Captured frame in {:.2?}", start.elapsed());

    let inference_start = Instant::now();
    let thought = brain.see_and_think(&frame, "Describe this image.")?;
    println!("Brain thought: '{}' in {:.2?}", thought, inference_start.elapsed());

    Ok(())
}