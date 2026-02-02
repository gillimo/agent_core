//! JSON validation for action intents and observations.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn ensure_object<'a>(value: &'a Value) -> Result<&'a serde_json::Map<String, Value>> {
    value
        .as_object()
        .ok_or_else(|| anyhow!("Expected JSON object"))
}

fn get_i64(obj: &serde_json::Map<String, Value>, key: &str) -> Result<i64> {
    obj.get(key)
        .and_then(|v| v.as_i64())
        .ok_or_else(|| anyhow!("Missing or invalid '{}'", key))
}

fn get_u64(obj: &serde_json::Map<String, Value>, key: &str) -> Result<u64> {
    obj.get(key)
        .and_then(|v| v.as_u64())
        .ok_or_else(|| anyhow!("Missing or invalid '{}'", key))
}

fn get_str<'a>(obj: &'a serde_json::Map<String, Value>, key: &str) -> Result<&'a str> {
    obj.get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| anyhow!("Missing or invalid '{}'", key))
}

fn validate_timing(obj: &serde_json::Map<String, Value>) -> Result<()> {
    if let Some(deadline) = obj.get("deadline_ms").and_then(|v| v.as_u64()) {
        if now_ms() > deadline {
            return Err(anyhow!("Deadline exceeded"));
        }
    }
    if let (Some(ts), Some(max_age)) = (
        obj.get("timestamp_ms").and_then(|v| v.as_u64()),
        obj.get("max_age_ms").and_then(|v| v.as_u64()),
    ) {
        if now_ms().saturating_sub(ts) > max_age {
            return Err(anyhow!("Action intent too old"));
        }
    }
    Ok(())
}

fn validate_action_fields(action: &str, obj: &serde_json::Map<String, Value>) -> Result<()> {
    match action {
        "move_mouse" => {
            let _x = get_i64(obj, "x")?;
            let _y = get_i64(obj, "y")?;
            Ok(())
        }
        "click" => {
            if let Some(button) = obj.get("button") {
                let s = button
                    .as_str()
                    .ok_or_else(|| anyhow!("Invalid 'button'"))?;
                let allowed = ["left", "right", "middle"];
                if !allowed.contains(&s.to_lowercase().as_str()) {
                    return Err(anyhow!("Invalid 'button'"));
                }
            }
            if obj.contains_key("x") || obj.contains_key("y") {
                let _x = get_i64(obj, "x")?;
                let _y = get_i64(obj, "y")?;
            }
            Ok(())
        }
        "press_key" => {
            let _key = get_str(obj, "key")?;
            Ok(())
        }
        "type_text" => {
            let _text = get_str(obj, "text")?;
            Ok(())
        }
        _ => Err(anyhow!("Unknown action: {}", action)),
    }
}

pub fn validate_action_intent_impl(action_json: &str) -> Value {
    let parsed: Value = match serde_json::from_str(action_json) {
        Ok(v) => v,
        Err(e) => return json!({ "valid": false, "error": format!("Invalid JSON: {}", e) }),
    };
    let obj = match ensure_object(&parsed) {
        Ok(o) => o,
        Err(e) => return json!({ "valid": false, "error": e.to_string() }),
    };
    if let Err(e) = validate_timing(obj) {
        return json!({ "valid": false, "error": e.to_string() });
    }

    let action = match obj.get("action").and_then(|v| v.as_str()) {
        Some(a) if !a.trim().is_empty() => a,
        _ => return json!({ "valid": false, "error": "Missing or invalid 'action'" }),
    };

    let result = validate_action_fields(action, obj);

    match result {
        Ok(()) => json!({ "valid": true }),
        Err(e) => json!({ "valid": false, "error": e.to_string() }),
    }
}

pub fn validate_snapshot_impl(snapshot_json: &str) -> Value {
    let parsed: Value = match serde_json::from_str(snapshot_json) {
        Ok(v) => v,
        Err(e) => return json!({ "valid": false, "error": format!("Invalid JSON: {}", e) }),
    };
    let obj = match ensure_object(&parsed) {
        Ok(o) => o,
        Err(e) => return json!({ "valid": false, "error": e.to_string() }),
    };

    let width = match obj.get("width").and_then(|v| v.as_u64()) {
        Some(v) if v > 0 => v,
        _ => return json!({ "valid": false, "error": "Missing or invalid 'width'" }),
    };
    let height = match obj.get("height").and_then(|v| v.as_u64()) {
        Some(v) if v > 0 => v,
        _ => return json!({ "valid": false, "error": "Missing or invalid 'height'" }),
    };
    let _timestamp = match obj.get("timestamp").and_then(|v| v.as_u64()) {
        Some(v) => v,
        _ => return json!({ "valid": false, "error": "Missing or invalid 'timestamp'" }),
    };

    let _ = width;
    let _ = height;

    if let Some(v) = obj.get("yellow_count") {
        if v.as_u64().is_none() {
            return json!({ "valid": false, "error": "Invalid 'yellow_count'" });
        }
    }
    if let Some(v) = obj.get("red_count") {
        if v.as_u64().is_none() {
            return json!({ "valid": false, "error": "Invalid 'red_count'" });
        }
    }

    if let Some(arrow) = obj.get("arrow") {
        if !arrow.is_null() {
            let arrow_obj = match arrow.as_object() {
                Some(a) => a,
                None => return json!({ "valid": false, "error": "Invalid 'arrow'" }),
            };
            if get_i64(arrow_obj, "x").is_err()
                || get_i64(arrow_obj, "y").is_err()
                || arrow_obj.get("confidence").and_then(|v| v.as_f64()).is_none()
            {
                return json!({ "valid": false, "error": "Invalid 'arrow' fields" });
            }
        }
    }

    if let Some(highlight) = obj.get("highlight") {
        if !highlight.is_null() {
            let highlight_obj = match highlight.as_object() {
                Some(h) => h,
                None => return json!({ "valid": false, "error": "Invalid 'highlight'" }),
            };
            if get_i64(highlight_obj, "x").is_err()
                || get_i64(highlight_obj, "y").is_err()
                || highlight_obj.get("confidence").and_then(|v| v.as_f64()).is_none()
            {
                return json!({ "valid": false, "error": "Invalid 'highlight' fields" });
            }
        }
    }

    if let Some(text) = obj.get("ocr_text") {
        if text.as_str().is_none() {
            return json!({ "valid": false, "error": "Invalid 'ocr_text'" });
        }
    }

    json!({ "valid": true })
}
