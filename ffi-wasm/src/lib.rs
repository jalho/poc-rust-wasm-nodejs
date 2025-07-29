#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get_one() -> u32 {
    return 1;
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn make_log_entry() -> LogEntry {
    LogEntry {
        instant: chrono::DateTime::from_timestamp_nanos(0),
    }
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct LogEntry {
    instant: chrono::DateTime<chrono::Utc>,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl LogEntry {
    pub fn as_timestamp(&self) -> i64 {
        return self.instant.timestamp_millis();
    }
}
