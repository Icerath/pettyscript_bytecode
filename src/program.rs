use crate::{builtins::Builtin, op_codes::OpCode, value::Value};
use std::ops::Deref;

#[derive(Debug, Default, Clone)]
pub struct Program {
    pub bytes: Vec<u8>,
    pub constants: Vec<Value>,
    pub idents: Vec<String>,
}

impl Program {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
    #[inline]
    pub fn load_const(&mut self, value: Value) -> usize {
        self.bytes.push(OpCode::LoadConst as u8);

        let index = insert_vec(&mut self.constants, value);

        let index_u32 = u32::try_from(index).unwrap();
        self.bytes.extend_from_slice(&index_u32.to_le_bytes());

        index
    }
    #[inline]
    pub fn push_literal<V: Into<Value>>(&mut self, value: V) -> usize {
        self.load_const(value.into())
    }
    #[inline]
    pub fn store_name(&mut self, name: impl Into<String>) -> usize {
        self.bytes.push(OpCode::StoreName as u8);
        let name = name.into();
        let index = insert_vec(&mut self.idents, name);

        let index_u32 = u32::try_from(index).unwrap();
        self.bytes.extend_from_slice(&index_u32.to_le_bytes());

        index
    }
    #[inline]
    pub fn load_name(&mut self, name: impl Into<String>) -> usize {
        self.bytes.push(OpCode::LoadName as u8);

        let name = name.into();
        let index = insert_vec(&mut self.idents, name);

        let index_u32 = u32::try_from(index).unwrap();
        self.bytes.extend_from_slice(&index_u32.to_le_bytes());

        index
    }
    #[inline]
    pub fn push_func<F>(&mut self, func: F) -> usize
    where
        F: FnOnce(&mut Self),
    {
        let jump_end = self.push_jump(0);
        let index = self.len();
        func(self);
        self.bytes.push(OpCode::Ret as u8);
        self.patch_jump(jump_end);
        index
    }
    #[inline]
    pub fn call_func(&mut self, func: usize) {
        self.bytes.push(OpCode::PrepareFuncCall as u8);
        self.push_jump(func);
    }
    #[inline]
    pub fn push_builtin(&mut self, builtin: Builtin) -> usize {
        self.bytes.push(OpCode::LoadBuiltin as u8);
        self.bytes.push(builtin as u8);
        self.len() - 2
    }
    /// # Panics
    /// Panics If `OpCode` has a non-zero size.
    #[inline]
    pub fn push_opcode(&mut self, opcode: OpCode) {
        assert_eq!(opcode.size_operand(), 0);
        self.bytes.push(opcode as u8);
    }
    #[inline]
    pub fn push_u32(&mut self, val: u32) {
        let bytes = val.to_le_bytes();
        self.bytes.extend_from_slice(&bytes);
    }
    #[inline]
    pub fn push_jump(&mut self, index: usize) -> usize {
        self.bytes.push(OpCode::Jump as u8);
        self.push_u32(u32::try_from(index).unwrap());
        self.len() - 4
    }
    #[inline]
    pub fn push_pop_jump_if_false(&mut self, index: usize) -> usize {
        self.bytes.push(OpCode::PopJumpIfFalse as u8);
        self.push_u32(u32::try_from(index).unwrap());
        self.len() - 4
    }
    #[inline]
    pub fn patch_jump(&mut self, jump: usize) {
        let here = self.len();
        let slice = &mut self.bytes[jump..jump + 4];
        slice.copy_from_slice(&u32::try_from(here).unwrap().to_le_bytes());
    }
    #[inline]
    pub fn push_if<F>(&mut self, body: F)
    where
        F: FnOnce(&mut Self),
    {
        let jump = self.push_pop_jump_if_false(0);
        body(self);
        self.patch_jump(jump);
    }

    #[inline]
    pub fn push_if_or_else<F1, F2>(&mut self, body: F1, orelse: F2)
    where
        F1: FnOnce(&mut Self),
        F2: FnOnce(&mut Self),
    {
        let jump_if = self.push_pop_jump_if_false(0);
        body(self);
        let jump_else = self.push_jump(0);
        self.patch_jump(jump_if);
        orelse(self);
        self.patch_jump(jump_else);
    }

    #[inline]
    pub fn push_while_loop<F1, F2>(&mut self, condition: F1, body: F2)
    where
        F1: FnOnce(&mut Self),
        F2: FnOnce(&mut Self),
    {
        let start = self.len();
        condition(self);
        let end = self.push_pop_jump_if_false(0);
        body(self);
        self.push_jump(start);
        self.patch_jump(end);
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

fn insert_vec<T: PartialOrd>(vec: &mut Vec<T>, value: T) -> usize {
    vec.iter().position(|val| val == &value).unwrap_or_else(|| {
        vec.push(value);
        vec.len() - 1
    })
}
