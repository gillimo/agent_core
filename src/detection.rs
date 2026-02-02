//! Fast pixel-based color detection
//!
//! Ported from AgentOSRS rust_core/src/detection.rs

use rayon::prelude::*;

/// Find yellow arrow (e.g., quest helper arrows)
/// Returns (x, y, confidence) if found
pub fn find_yellow_arrow_impl(img_data: &[u8], width: u32, height: u32) -> Option<(i32, i32, f32)> {
    // Yellow arrow: R > 200, G > 200, B < 80
    let pixels_per_row = width as usize;
    let total_pixels = (width * height) as usize;

    if img_data.len() < total_pixels * 4 {
        return None;
    }

    let yellow_pixels: Vec<(usize, usize)> = (0..total_pixels)
        .into_par_iter()
        .filter_map(|i| {
            let offset = i * 4;
            let r = img_data[offset];
            let g = img_data[offset + 1];
            let b = img_data[offset + 2];

            if r > 200 && g > 200 && b < 80 {
                let x = i % pixels_per_row;
                let y = i / pixels_per_row;
                Some((x, y))
            } else {
                None
            }
        })
        .collect();

    if yellow_pixels.len() < 10 {
        return None;
    }

    let sum_x: usize = yellow_pixels.iter().map(|(x, _)| x).sum();
    let sum_y: usize = yellow_pixels.iter().map(|(_, y)| y).sum();
    let count = yellow_pixels.len();

    let center_x = (sum_x / count) as i32;
    let center_y = (sum_y / count) as i32;
    let confidence = (count as f32 / 500.0).min(1.0);

    Some((center_x, center_y, confidence))
}

/// Find cyan highlight (e.g., interactive object highlights)
/// Returns (x, y, confidence) if found
pub fn find_cyan_highlight_impl(img_data: &[u8], width: u32, height: u32) -> Option<(i32, i32, f32)> {
    // Cyan: R < 80, G > 180, B > 180
    let pixels_per_row = width as usize;
    let total_pixels = (width * height) as usize;

    if img_data.len() < total_pixels * 4 {
        return None;
    }

    let cyan_pixels: Vec<(usize, usize)> = (0..total_pixels)
        .into_par_iter()
        .filter_map(|i| {
            let offset = i * 4;
            let r = img_data[offset];
            let g = img_data[offset + 1];
            let b = img_data[offset + 2];

            if r < 80 && g > 180 && b > 180 {
                let x = i % pixels_per_row;
                let y = i / pixels_per_row;
                Some((x, y))
            } else {
                None
            }
        })
        .collect();

    if cyan_pixels.len() < 20 {
        return None;
    }

    let sum_x: usize = cyan_pixels.iter().map(|(x, _)| x).sum();
    let sum_y: usize = cyan_pixels.iter().map(|(_, y)| y).sum();
    let count = cyan_pixels.len();

    let center_x = (sum_x / count) as i32;
    let center_y = (sum_y / count) as i32;
    let confidence = (count as f32 / 1000.0).min(1.0);

    Some((center_x, center_y, confidence))
}

/// Detect pixels matching a specific color within tolerance
/// Returns list of (x, y) coordinates
pub fn detect_color_impl(
    img_data: &[u8],
    width: u32,
    height: u32,
    target_r: u8,
    target_g: u8,
    target_b: u8,
    tolerance: u8,
) -> Vec<(i32, i32)> {
    let pixels_per_row = width as usize;
    let total_pixels = (width * height) as usize;

    if img_data.len() < total_pixels * 4 {
        return Vec::new();
    }

    let tol = tolerance as i16;

    (0..total_pixels)
        .into_par_iter()
        .filter_map(|i| {
            let offset = i * 4;
            let r = img_data[offset] as i16;
            let g = img_data[offset + 1] as i16;
            let b = img_data[offset + 2] as i16;

            let dr = (r - target_r as i16).abs();
            let dg = (g - target_g as i16).abs();
            let db = (b - target_b as i16).abs();

            if dr <= tol && dg <= tol && db <= tol {
                let x = (i % pixels_per_row) as i32;
                let y = (i / pixels_per_row) as i32;
                Some((x, y))
            } else {
                None
            }
        })
        .collect()
}
