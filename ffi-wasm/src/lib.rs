#[wasm_bindgen::prelude::wasm_bindgen]
pub fn get_one() -> u32 {
    return 1;
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn make_log_entry_from_nanos(nanos: i64) -> LogEntry {
    LogEntry::new(nanos)
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub fn deserialize_log_entry_from_json(serialized: String) -> Result<LogEntry, String> {
    let deserialized: LogEntryJson = match serde_json::from_str::<LogEntryJson>(&serialized) {
        Ok(n) => n,
        Err(err) => return Err(err.to_string()),
    };
    Ok(LogEntry::new(deserialized.nanos))
}

#[derive(serde::Deserialize, serde::Serialize)]
struct LogEntryJson {
    nanos: i64,
}

#[wasm_bindgen::prelude::wasm_bindgen]
pub struct LogEntry {
    #[wasm_bindgen(readonly)]
    pub nanos: i64,
    micros: i64,

    instant: chrono::DateTime<chrono::Utc>,
}

#[wasm_bindgen::prelude::wasm_bindgen]
impl LogEntry {
    pub fn new(nanos: i64) -> Self {
        Self {
            nanos,
            micros: nanos / 1000,
            instant: chrono::DateTime::from_timestamp_nanos(nanos),
        }
    }

    pub fn as_timestamp_millis(&self) -> i64 {
        return self.instant.timestamp_millis();
    }

    pub fn as_timestamp_micros(&self) -> i64 {
        self.read_private_micros()
    }

    /// Returns an RFC 3339 and ISO 8601 date and time string such as
    /// `1996-12-19T16:39:57-08:00`.
    pub fn as_timestamp_rfc3339(&self) -> String {
        return self.instant.to_rfc3339();
    }

    fn read_private_micros(&self) -> i64 {
        self.micros
    }

    pub fn serialize_to_json(&self) -> Result<String, String> {
        let serializable: LogEntryJson = LogEntryJson { nanos: self.nanos };
        match serde_json::to_string(&serializable) {
            Ok(n) => Ok(n),
            Err(err) => Err(err.to_string()),
        }
    }
}
