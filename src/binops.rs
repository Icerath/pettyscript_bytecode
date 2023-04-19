#![allow(clippy::cast_precision_loss)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

use std::{
    borrow::Cow,
    ops::{Add, Div, Mul, Sub},
};

use crate::value::Value;

impl Add for Value {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs + rhs),
            (Self::Float(lhs), Self::Int(rhs)) => Self::Float(lhs + rhs as f64),
            (Self::Int(lhs), Self::Float(rhs)) => Self::Float(lhs as f64 + rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs + rhs),
            (Self::Str(lhs), Self::Str(rhs)) => Self::Str(lhs + rhs),
            (lhs, rhs) => todo!("{lhs} + {rhs}"),
        }
    }
}

impl Sub for Value {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs - rhs),
            (Self::Float(lhs), Self::Int(rhs)) => Self::Float(lhs - rhs as f64),
            (Self::Int(lhs), Self::Float(rhs)) => Self::Float(lhs as f64 - rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs - rhs),
            (lhs, rhs) => todo!("{lhs} - {rhs}"),
        }
    }
}

impl Mul for Value {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Int(lhs * rhs),
            (Self::Float(lhs), Self::Int(rhs)) => Self::Float(lhs * rhs as f64),
            (Self::Int(lhs), Self::Float(rhs)) => Self::Float(lhs as f64 * rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs * rhs),
            (Self::Str(str), Self::Int(int)) | (Self::Int(int), Self::Str(str)) => {
                if int.is_positive() {
                    return Value::Str(Cow::Owned(str.repeat(int as usize)));
                }
                Value::Str(str)
            }
            (lhs, rhs) => todo!("{lhs} - {rhs}"),
        }
    }
}

impl Div for Value {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Self::Int(lhs), Self::Int(rhs)) => Self::Float(lhs as f64 / rhs as f64),
            (Self::Float(lhs), Self::Int(rhs)) => Self::Float(lhs / rhs as f64),
            (Self::Int(lhs), Self::Float(rhs)) => Self::Float(lhs as f64 / rhs),
            (Self::Float(lhs), Self::Float(rhs)) => Self::Float(lhs / rhs),
            (lhs, rhs) => todo!("{lhs} - {rhs}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(Value::Int(1) + Value::Int(2), Value::Int(3));
        assert_eq!(Value::Float(1.5) + Value::Int(2), Value::Float(3.5));
        assert_eq!(Value::Int(1) + Value::Float(0.5), Value::Float(1.5));
        assert_eq!(Value::Float(1.5) + Value::Float(2.5), Value::Float(4.0));
    }
    #[test]
    fn test_sub() {
        assert_eq!(Value::Int(5) - Value::Int(3), Value::Int(2));
        assert_eq!(Value::Float(3.5) - Value::Int(1), Value::Float(2.5));
        assert_eq!(Value::Int(1) - Value::Float(0.5), Value::Float(0.5));
        assert_eq!(Value::Float(4.5) - Value::Float(2.5), Value::Float(2.0));
    }
    #[test]
    fn test_mul() {
        assert_eq!(Value::Int(5) * Value::Int(3), Value::Int(15));
        assert_eq!(Value::Float(3.5) * Value::Int(2), Value::Float(7.0));
        assert_eq!(Value::Int(1) * Value::Float(0.5), Value::Float(0.5));
        assert_eq!(Value::Float(5.0) * Value::Float(2.5), Value::Float(12.5));
    }
    #[test]
    fn test_div() {
        assert_eq!(Value::Int(7) / Value::Int(2), Value::Float(3.5));
        assert_eq!(Value::Float(3.5) / Value::Int(2), Value::Float(1.75));
        assert_eq!(Value::Int(4) / Value::Float(2.0), Value::Float(2.0));
        assert_eq!(Value::Float(5.0) / Value::Float(2.5), Value::Float(2.0));
    }
}
