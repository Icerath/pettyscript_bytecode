use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Float(f64),
    Str(Cow<'static, str>),
    Function(usize),
}
