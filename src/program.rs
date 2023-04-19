use crate::value::Value;
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
    #[must_use]
    #[inline]
    pub fn read_arr<const LEN: usize>(&self, from: usize) -> Option<[u8; LEN]> {
        let slice = self.bytes.get(from..LEN)?;
        TryFrom::try_from(slice).ok()
    }
}

impl Deref for Program {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.bytes
    }
}
