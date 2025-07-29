#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get_one() -> u32 {
    return 1;
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn make_log_entry(nanos: i64) -> LogEntry {
    LogEntry::new(nanos)
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct LogEntry {
    pub nanos: i64,

    instant: chrono::DateTime<chrono::Utc>,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl LogEntry {
    pub fn new(nanos: i64) -> Self {
        Self {
            nanos,
            instant: chrono::DateTime::from_timestamp_nanos(nanos),
        }
    }

    pub fn as_timestamp_millis(&self) -> i64 {
        return self.instant.timestamp_millis();
    }

    pub fn as_timestamp_rfc3339(&self) -> String {
        return self.instant.to_rfc3339();
    }
}
