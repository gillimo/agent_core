//! Simple in-memory OCR text recorder.
use std::sync::{Mutex, OnceLock};

const MAX_RECORDS: usize = 1000;

static RECORDS: OnceLock<Mutex<Vec<String>>> = OnceLock::new();

fn storage() -> &'static Mutex<Vec<String>> {
    RECORDS.get_or_init(|| Mutex::new(Vec::new()))
}

pub fn record_text(line: String) {
    let mut guard = storage().lock().unwrap();
    guard.push(line);
    if guard.len() > MAX_RECORDS {
        let overflow = guard.len() - MAX_RECORDS;
        guard.drain(0..overflow);
    }
}

pub fn get_records(limit: Option<usize>) -> Vec<String> {
    let guard = storage().lock().unwrap();
    match limit {
        Some(n) if n < guard.len() => guard[guard.len() - n..].to_vec(),
        _ => guard.clone(),
    }
}

pub fn clear_records() {
    let mut guard = storage().lock().unwrap();
    guard.clear();
}
