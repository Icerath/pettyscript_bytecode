#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
pub mod dis;
pub mod op_codes;
pub mod program;
pub mod value;

#[cfg(test)]
mod tests;
