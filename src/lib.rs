//! Agent Core - Eyes and Hands for AI agents
//!
//! A smart function library that executes commands and returns observations.
//! The agent code directs these tools - agent_core just executes.

use pyo3::prelude::*;

mod eye;
mod capture;
mod detection;
mod input;
mod brain;
mod ocr;
mod validation;
mod record;

use capture::{
    capture_region_impl,
    capture_full_screen_impl,
    capture_window_by_title_impl,
    focus_window_by_title_impl,
    capture_window_by_all_impl,
    focus_window_by_all_impl,
};
use detection::{detect_color_impl, find_yellow_arrow_impl, find_cyan_highlight_impl};
use input::{move_mouse_impl, click_impl, type_text_impl, press_key_impl};
use ocr::{ocr_region_impl, ocr_regions_impl};
use validation::{validate_action_intent_impl, validate_snapshot_impl};
use record::{record_text, get_records, clear_records};

// JSON API

#[pyfunction]
fn execute_action(action_json: &str) -> PyResult<String> {
    let parsed: serde_json::Value = serde_json::from_str(action_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid JSON: {}", e)))?;
    let action = parsed.get("action").and_then(|v| v.as_str()).unwrap_or("");
    let result = match action {
        "press_key" => {
            let key = parsed.get("key").and_then(|v| v.as_str()).unwrap_or("");
            match press_key_impl(key) {
                Ok(()) => serde_json::json!({"success": true}),
                Err(e) => serde_json::json!({"success": false, "error": e.to_string()}),
            }
        }
        "click" => {
            let button = parsed.get("button").and_then(|v| v.as_str()).unwrap_or("left");
            let x = parsed.get("x").and_then(|v| v.as_i64()).map(|v| v as i32);
            let y = parsed.get("y").and_then(|v| v.as_i64()).map(|v| v as i32);
            match click_impl(button, x, y) {
                Ok(()) => serde_json::json!({"success": true}),
                Err(e) => serde_json::json!({"success": false, "error": e.to_string()}),
            }
        }
        "move_mouse" => {
            let x = parsed.get("x").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let y = parsed.get("y").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            match move_mouse_impl(x, y) {
                Ok(()) => serde_json::json!({"success": true}),
                Err(e) => serde_json::json!({"success": false, "error": e.to_string()}),
            }
        }
        "type_text" => {
            let text = parsed.get("text").and_then(|v| v.as_str()).unwrap_or("");
            match type_text_impl(text) {
                Ok(()) => serde_json::json!({"success": true}),
                Err(e) => serde_json::json!({"success": false, "error": e.to_string()}),
            }
        }
        "detect_color" => {
            let r = parsed.get("r").and_then(|v| v.as_u64()).unwrap_or(255) as u8;
            let g = parsed.get("g").and_then(|v| v.as_u64()).unwrap_or(255) as u8;
            let b = parsed.get("b").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
            let tol = parsed.get("tolerance").and_then(|v| v.as_u64()).unwrap_or(30) as u8;
            match capture_full_screen_impl() {
                Ok((w, h, data)) => {
                    let matches = detect_color_impl(&data, w, h, r, g, b, tol);
                    serde_json::json!({"success": true, "count": matches.len()})
                }
                Err(e) => serde_json::json!({"success": false, "error": e.to_string()}),
            }
        }
        _ => serde_json::json!({"success": false, "error": format!("Unknown action: {}", action)}),
    };
    Ok(result.to_string())
}

#[pyfunction]
fn get_observation() -> PyResult<String> {
    let (width, height, data) = capture_full_screen_impl()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let arrow = find_yellow_arrow_impl(&data, width, height);
    let highlight = find_cyan_highlight_impl(&data, width, height);
    let yellow = detect_color_impl(&data, width, height, 248, 208, 48, 40);
    let red = detect_color_impl(&data, width, height, 248, 56, 32, 30);
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
    let obs = serde_json::json!({
        "width": width, "height": height,
        "yellow_count": yellow.len(), "red_count": red.len(),
        "arrow": arrow.map(|(x, y, c)| serde_json::json!({"x": x, "y": y, "confidence": c})),
        "highlight": highlight.map(|(x, y, c)| serde_json::json!({"x": x, "y": y, "confidence": c})),
        "timestamp": timestamp
    });
    Ok(obs.to_string())
}

// Direct API

#[pyfunction]
fn capture_region(x: i32, y: i32, width: u32, height: u32) -> PyResult<Vec<u8>> {
    capture_region_impl(x, y, width, height)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn capture_screen() -> PyResult<(u32, u32, Vec<u8>)> {
    capture_full_screen_impl()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn detect_arrow(img_data: Vec<u8>, width: u32, height: u32) -> PyResult<Option<(i32, i32, f32)>> {
    Ok(find_yellow_arrow_impl(&img_data, width, height))
}

#[pyfunction]
fn detect_highlight(img_data: Vec<u8>, width: u32, height: u32) -> PyResult<Option<(i32, i32, f32)>> {
    Ok(find_cyan_highlight_impl(&img_data, width, height))
}

#[pyfunction]
fn detect_color(img_data: Vec<u8>, width: u32, height: u32, r: u8, g: u8, b: u8, tolerance: u8) -> PyResult<Vec<(i32, i32)>> {
    Ok(detect_color_impl(&img_data, width, height, r, g, b, tolerance))
}

#[pyfunction]
fn ocr_region(
    img_data: Vec<u8>,
    img_width: u32,
    img_height: u32,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> PyResult<String> {
    ocr_region_impl(&img_data, img_width, img_height, x, y, width, height)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn ocr_regions(
    img_data: Vec<u8>,
    img_width: u32,
    img_height: u32,
    regions: Vec<(i32, i32, u32, u32)>,
) -> PyResult<Vec<String>> {
    ocr_regions_impl(&img_data, img_width, img_height, &regions)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn ocr_window_region(
    title_contains: &str,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
) -> PyResult<String> {
    focus_window_by_title_impl(title_contains)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let (w, h, data) = capture_window_by_title_impl(title_contains)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    ocr_region_impl(&data, w, h, x, y, width, height)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn ocr_window_full(title_contains: &str) -> PyResult<String> {
    focus_window_by_title_impl(title_contains)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let (w, h, data) = capture_window_by_title_impl(title_contains)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    ocr_region_impl(&data, w, h, 0, 0, w, h)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn ocr_window_region_all(title_parts: Vec<String>, x: i32, y: i32, width: u32, height: u32) -> PyResult<String> {
    focus_window_by_all_impl(&title_parts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let (w, h, data) = capture_window_by_all_impl(&title_parts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    ocr_region_impl(&data, w, h, x, y, width, height)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn ocr_window_full_all(title_parts: Vec<String>) -> PyResult<String> {
    focus_window_by_all_impl(&title_parts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let (w, h, data) = capture_window_by_all_impl(&title_parts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    ocr_region_impl(&data, w, h, 0, 0, w, h)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn ocr_window_full_all_record(title_parts: Vec<String>, suppress_json: &str) -> PyResult<String> {
    let window = focus_window_by_all_impl(&title_parts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
    let (w, h, data) = capture_window_by_all_impl(&title_parts)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let text = ocr_region_impl(&data, w, h, 0, 0, w, h)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    let cfg: serde_json::Value = serde_json::from_str(suppress_json)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid suppress JSON: {}", e)))?;

    let mut suppressed = false;
    let mut reasons: Vec<String> = Vec::new();
    let mut keyword_hits: u64 = 0;
    let mut color_hits: u64 = 0;

    if let Some(keywords) = cfg.get("keywords").and_then(|v| v.as_array()) {
        let case_insensitive = cfg.get("case_insensitive").and_then(|v| v.as_bool()).unwrap_or(true);
        let min_hits = cfg.get("min_keyword_hits").and_then(|v| v.as_u64()).unwrap_or(1);
        let text_cmp = if case_insensitive { text.to_lowercase() } else { text.clone() };
        for kw in keywords {
            if let Some(s) = kw.as_str() {
                let needle = if case_insensitive { s.to_lowercase() } else { s.to_string() };
                if !needle.is_empty() && text_cmp.contains(&needle) {
                    keyword_hits += 1;
                }
            }
        }
        if keyword_hits >= min_hits {
            suppressed = true;
            reasons.push("keyword_match".to_string());
        }
    }

    if let Some(color) = cfg.get("color").and_then(|v| v.as_object()) {
        let r = color.get("r").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let g = color.get("g").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let b = color.get("b").and_then(|v| v.as_u64()).unwrap_or(0) as u8;
        let tolerance = color.get("tolerance").and_then(|v| v.as_u64()).unwrap_or(30) as u8;
        let min_count = color.get("min_count").and_then(|v| v.as_u64()).unwrap_or(1);
        let matches = detect_color_impl(&data, w, h, r, g, b, tolerance);
        color_hits = matches.len() as u64;
        if color_hits >= min_count {
            suppressed = true;
            reasons.push("color_match".to_string());
        }
    }

    let trimmed = text.trim().to_string();
    let mut recorded = false;
    if !suppressed && !trimmed.is_empty() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH).map(|d| d.as_millis() as u64).unwrap_or(0);
        let line = format!("[{}] {}", timestamp, trimmed);
        record_text(line);
        recorded = true;
    }

    let response = serde_json::json!({
        "text": text,
        "recorded": recorded,
        "suppressed": suppressed,
        "reasons": reasons,
        "keyword_hits": keyword_hits,
        "color_hits": color_hits,
        "window_title": window.1
    });
    Ok(response.to_string())
}

#[pyfunction]
#[pyo3(signature = (limit=None))]
fn get_recorded_text(limit: Option<usize>) -> PyResult<Vec<String>> {
    Ok(get_records(limit))
}

#[pyfunction]
fn clear_recorded_text() -> PyResult<()> {
    clear_records();
    Ok(())
}

#[pyfunction]
fn validate_action_intent(action_json: &str) -> PyResult<String> {
    Ok(validate_action_intent_impl(action_json).to_string())
}

#[pyfunction]
fn validate_snapshot(snapshot_json: &str) -> PyResult<String> {
    Ok(validate_snapshot_impl(snapshot_json).to_string())
}

#[pyfunction]
fn move_mouse(x: i32, y: i32) -> PyResult<()> {
    move_mouse_impl(x, y).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
#[pyo3(signature = (button, x=None, y=None))]
fn click(button: &str, x: Option<i32>, y: Option<i32>) -> PyResult<()> {
    click_impl(button, x, y).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn type_text(text: &str) -> PyResult<()> {
    type_text_impl(text).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn press_key(key: &str) -> PyResult<()> {
    press_key_impl(key).map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))
}

#[pyfunction]
fn map_coordinates(ai_x: i32, ai_y: i32, ai_width: u32, ai_height: u32, screen_width: u32, screen_height: u32) -> (i32, i32) {
    let scale_x = screen_width as f32 / ai_width as f32;
    let scale_y = screen_height as f32 / ai_height as f32;
    ((ai_x as f32 * scale_x) as i32, (ai_y as f32 * scale_y) as i32)
}

#[pyfunction]
fn version() -> &'static str { env!("CARGO_PKG_VERSION") }

#[pymodule]
fn agent_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(execute_action, m)?)?;
    m.add_function(wrap_pyfunction!(get_observation, m)?)?;
    m.add_function(wrap_pyfunction!(capture_region, m)?)?;
    m.add_function(wrap_pyfunction!(capture_screen, m)?)?;
    m.add_function(wrap_pyfunction!(detect_arrow, m)?)?;
    m.add_function(wrap_pyfunction!(detect_highlight, m)?)?;
    m.add_function(wrap_pyfunction!(detect_color, m)?)?;
    m.add_function(wrap_pyfunction!(ocr_region, m)?)?;
    m.add_function(wrap_pyfunction!(ocr_regions, m)?)?;
    m.add_function(wrap_pyfunction!(ocr_window_region, m)?)?;
    m.add_function(wrap_pyfunction!(ocr_window_full, m)?)?;
    m.add_function(wrap_pyfunction!(ocr_window_region_all, m)?)?;
    m.add_function(wrap_pyfunction!(ocr_window_full_all, m)?)?;
    m.add_function(wrap_pyfunction!(ocr_window_full_all_record, m)?)?;
    m.add_function(wrap_pyfunction!(get_recorded_text, m)?)?;
    m.add_function(wrap_pyfunction!(clear_recorded_text, m)?)?;
    m.add_function(wrap_pyfunction!(validate_action_intent, m)?)?;
    m.add_function(wrap_pyfunction!(validate_snapshot, m)?)?;
    m.add_function(wrap_pyfunction!(move_mouse, m)?)?;
    m.add_function(wrap_pyfunction!(click, m)?)?;
    m.add_function(wrap_pyfunction!(type_text, m)?)?;
    m.add_function(wrap_pyfunction!(press_key, m)?)?;
    m.add_function(wrap_pyfunction!(map_coordinates, m)?)?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    Ok(())
}
