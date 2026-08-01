use wasm_bindgen::prelude::*;
use serde_json::{Value as JsonValue, Map};
use tomli_rust::{Value as TomlValue, Datetime};

#[wasm_bindgen]
pub fn parse_toml(input: &str) -> String {
    match tomli_rust::parse(input) {
        Ok(table) => {
            let mut map = Map::new();
            for (k, v) in table {
                map.insert(k, to_json_value(v));
            }
            serde_json::to_string_pretty(&JsonValue::Object(map)).unwrap_or_else(|_| "{}".to_string())
        }
        Err(e) => {
            // Return error object as JSON
            let mut err_map = Map::new();
            err_map.insert("error".to_string(), JsonValue::String(e.to_string()));
            serde_json::to_string_pretty(&JsonValue::Object(err_map)).unwrap()
        }
    }
}

fn to_json_value(val: TomlValue) -> JsonValue {
    match val {
        TomlValue::String(s) => JsonValue::String(s),
        TomlValue::Integer(i) => JsonValue::Number(serde_json::Number::from(i)),
        TomlValue::Float(f) => {
            if let Some(num) = serde_json::Number::from_f64(f) {
                JsonValue::Number(num)
            } else {
                JsonValue::Null // Handle NaN/Infinity if they occur, though JSON doesn't support them
            }
        },
        TomlValue::Boolean(b) => JsonValue::Bool(b),
        TomlValue::Array(arr) => {
            let vec: Vec<JsonValue> = arr.into_iter().map(to_json_value).collect();
            JsonValue::Array(vec)
        },
        TomlValue::Table(table) => {
            let mut map = Map::new();
            for (k, v) in table {
                map.insert(k, to_json_value(v));
            }
            JsonValue::Object(map)
        },
        TomlValue::Datetime(dt) => {
            // Simple stringification for now, using a format that approximates it.
            // A more robust implementation would use a proper formatter for the Datetime structs.
            let dt_str = match dt {
                Datetime::OffsetDateTime { date, time, offset } => {
                    let sign = if offset.sign >= 0 { '+' } else { '-' };
                    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}{}{:02}:{:02}", 
                        date.year, date.month, date.day, 
                        time.hour, time.minute, time.second, time.microsecond,
                        sign, offset.hour, offset.minute)
                },
                Datetime::LocalDateTime { date, time } => {
                    format!("{:04}-{:02}-{:02}T{:02}:{:02}:{:02}.{:06}", 
                        date.year, date.month, date.day, 
                        time.hour, time.minute, time.second, time.microsecond)
                },
                Datetime::LocalDate(date) => {
                    format!("{:04}-{:02}-{:02}", date.year, date.month, date.day)
                },
                Datetime::LocalTime(time) => {
                    format!("{:02}:{:02}:{:02}.{:06}", time.hour, time.minute, time.second, time.microsecond)
                }
            };
            JsonValue::String(dt_str)
        }
    }
}
