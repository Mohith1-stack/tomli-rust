use std::collections::BTreeMap;

pub use crate::datetime::Datetime;

/// Represents a parsed TOML table (a map of string keys to TOML values).
pub type Table = BTreeMap<String, Value>;

/// Represents any valid TOML value.
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    /// A TOML string (basic, literal, or multiline).
    String(String),
    /// A TOML integer (64-bit signed).
    Integer(i64),
    /// A TOML float (64-bit IEEE 754).
    Float(f64),
    /// A TOML boolean (`true` or `false`).
    Boolean(bool),
    /// A TOML datetime.
    Datetime(Datetime),
    /// A TOML array containing a list of values.
    Array(Vec<Value>),
    /// A TOML inline table or standard table.
    Table(Table),
}

impl Value {
    /// Returns the type name for error formatting.
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::String(_) => "string",
            Value::Integer(_) => "integer",
            Value::Float(_) => "float",
            Value::Boolean(_) => "boolean",
            Value::Datetime(_) => "datetime",
            Value::Array(_) => "array",
            Value::Table(_) => "table",
        }
    }

    pub fn as_table_mut(&mut self) -> Option<&mut Table> {
        if let Value::Table(t) = self {
            Some(t)
        } else {
            None
        }
    }

    pub fn as_array_mut(&mut self) -> Option<&mut Vec<Value>> {
        if let Value::Array(a) = self {
            Some(a)
        } else {
            None
        }
    }
}
