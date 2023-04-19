use crate::{op_codes::OpCode, value::Value};
use std::ops::Deref;

#[derive(Debug, Default, Clone)]
pub struct Program {
    pub bytes: Vec<u8>,
    pub constants: Vec<Value>,
}

impl Program {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn load_const(&mut self, value: Value) -> usize {
        self.bytes.push(OpCode::LoadConst as u8);

        let index = self
            .constants
            .iter()
            .position(|val| val == &value)
            .unwrap_or_else(|| {
                self.constants.push(value);
                self.constants.len() - 1
            });

        let index_u32 = u32::try_from(index).unwrap();
        self.bytes.extend_from_slice(&index_u32.to_le_bytes());

        index
    }
    #[inline]
    pub fn push_literal<V: Into<Value>>(&mut self, value: V) -> usize {
        self.load_const(value.into())
    }
    /// # Panics
    /// Panics If `OpCode` has a non-zero size.
    #[inline]
    pub fn push_opcode(&mut self, opcode: OpCode) {
        assert_eq!(opcode.size_operand(), 0);
        self.bytes.push(opcode as u8);
    }
    #[must_use]
    #[inline]
    pub fn read_arr<const LEN: usize>(&self, from: usize) -> Option<[u8; LEN]> {
        let slice = self.bytes.get(from..from + LEN)?;
        TryFrom::try_from(slice).ok()
    }
}

impl Deref for Program {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
