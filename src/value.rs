use std::{borrow::Cow, fmt};

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(Cow<'static, str>),
    Function(usize),
}

#[derive(Debug)]
pub struct InvalidBool;
impl TryFrom<&Value> for bool {
    type Error = InvalidBool;
    fn try_from(value: &Value) -> Result<Self, Self::Error> {
        match value {
            Value::Int(int) => Ok(*int != 0),
            Value::Str(str) => Ok(str.is_empty()),
            Value::Float(float) => Ok(*float != 0.0),
            Value::Function(_) => Err(InvalidBool),
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
            Self::Function(index) => write!(f, "func({index})"),
        }
    }
}
