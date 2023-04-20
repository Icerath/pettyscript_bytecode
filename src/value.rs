use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(Cow<'static, str>),
}

impl From<&Value> for bool {
    fn from(value: &Value) -> Self {
        match value {
            Value::Int(int) => *int != 0,
            Value::Str(str) => str.is_empty(),
            Value::Float(float) => *float != 0.0,
        }
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::Float(value)
    }
}

impl From<&'static str> for Value {
    fn from(value: &'static str) -> Self {
        Self::Str(Cow::Borrowed(value))
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Str(Cow::Owned(value))
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Int(i64::from(value))
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int(int) => write!(f, "{int}"),
            Self::Float(float) => write!(f, "{float}"),
            Self::Str(str) => write!(f, "'{str}'"),
        }
    }
}
