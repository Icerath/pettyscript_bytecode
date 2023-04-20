#![warn(clippy::pedantic)]
#![allow(clippy::missing_panics_doc)]
pub mod assembler;
pub mod binops;
mod cursor;
pub mod dis;
pub mod op_codes;
pub mod program;
pub mod value;
pub mod vm;

#[cfg(test)]
mod tests;
