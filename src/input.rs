//! Input control module - Mouse and keyboard control

use anyhow::Result;
use enigo::{Enigo, Key, Keyboard, Mouse, Settings, Direction, Coordinate, Button};
use std::thread;
use std::time::Duration;

fn get_key(key: &str) -> Result<Key> {
    let k = match key.to_lowercase().as_str() {
        "return" | "enter" => Key::Return,
        "escape" | "esc" => Key::Escape,
        "tab" => Key::Tab,
        "space" => Key::Space,
        "backspace" => Key::Backspace,
        "delete" => Key::Delete,
        "up" => Key::UpArrow,
        "down" => Key::DownArrow,
        "left" => Key::LeftArrow,
        "right" => Key::RightArrow,
        "home" => Key::Home,
        "end" => Key::End,
        "pageup" => Key::PageUp,
        "pagedown" => Key::PageDown,
        "f1" => Key::F1, "f2" => Key::F2, "f3" => Key::F3, "f4" => Key::F4,
        "f5" => Key::F5, "f6" => Key::F6, "f7" => Key::F7, "f8" => Key::F8,
        "f9" => Key::F9, "f10" => Key::F10, "f11" => Key::F11, "f12" => Key::F12,
        "shift" => Key::Shift,
        "control" | "ctrl" => Key::Control,
        "alt" => Key::Alt,
        _ => {
            if key.len() == 1 {
                Key::Unicode(key.chars().next().unwrap())
            } else {
                return Err(anyhow::anyhow!("Unknown key: {}", key));
            }
        }
    };
    Ok(k)
}

/// Move mouse to absolute position
pub fn move_mouse_impl(x: i32, y: i32) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| anyhow::anyhow!("Failed to create Enigo: {:?}", e))?;
    enigo.move_mouse(x, y, Coordinate::Abs)
        .map_err(|e| anyhow::anyhow!("Failed to move mouse: {:?}", e))?;
    Ok(())
}

/// Click mouse button
pub fn click_impl(button: &str, x: Option<i32>, y: Option<i32>) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| anyhow::anyhow!("Failed to create Enigo: {:?}", e))?;
    if let (Some(px), Some(py)) = (x, y) {
        enigo.move_mouse(px, py, Coordinate::Abs)
            .map_err(|e| anyhow::anyhow!("Failed to move mouse: {:?}", e))?;
    }
    let btn = match button.to_lowercase().as_str() {
        "left" => Button::Left,
        "right" => Button::Right,
        "middle" => Button::Middle,
        _ => return Err(anyhow::anyhow!("Unknown button: {}", button)),
    };
    enigo.button(btn, Direction::Click)
        .map_err(|e| anyhow::anyhow!("Failed to click: {:?}", e))?;
    Ok(())
}

/// Type text string
pub fn type_text_impl(text: &str) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| anyhow::anyhow!("Failed to create Enigo: {:?}", e))?;
    enigo.text(text)
        .map_err(|e| anyhow::anyhow!("Failed to type text: {:?}", e))?;
    Ok(())
}

/// Press a key (tap)
pub fn press_key_impl(key: &str) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| anyhow::anyhow!("Failed to create Enigo: {:?}", e))?;
    let k = get_key(key)?;
    enigo.key(k, Direction::Click)
        .map_err(|e| anyhow::anyhow!("Failed to press key: {:?}", e))?;
    Ok(())
}

/// Hold a key for specified duration (for walking, etc)
pub fn hold_key_impl(key: &str, duration_ms: u64) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| anyhow::anyhow!("Failed to create Enigo: {:?}", e))?;
    let k = get_key(key)?;
    
    // Key down
    enigo.key(k, Direction::Press)
        .map_err(|e| anyhow::anyhow!("Failed to press key down: {:?}", e))?;
    
    // Hold
    thread::sleep(Duration::from_millis(duration_ms));
    
    // Key up
    enigo.key(k, Direction::Release)
        .map_err(|e| anyhow::anyhow!("Failed to release key: {:?}", e))?;
    
    Ok(())
}

/// Key down (for manual control)
pub fn key_down_impl(key: &str) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| anyhow::anyhow!("Failed to create Enigo: {:?}", e))?;
    let k = get_key(key)?;
    enigo.key(k, Direction::Press)
        .map_err(|e| anyhow::anyhow!("Failed to press key down: {:?}", e))?;
    Ok(())
}

/// Key up (for manual control)
pub fn key_up_impl(key: &str) -> Result<()> {
    let mut enigo = Enigo::new(&Settings::default())
        .map_err(|e| anyhow::anyhow!("Failed to create Enigo: {:?}", e))?;
    let k = get_key(key)?;
    enigo.key(k, Direction::Release)
        .map_err(|e| anyhow::anyhow!("Failed to release key: {:?}", e))?;
    Ok(())
}
