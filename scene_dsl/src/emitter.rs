use serde_json::Value;

pub fn emit(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap()
}
