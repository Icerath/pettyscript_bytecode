#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Builtin {
    Print,

    // Must be highest value
    Exit,
}

#[derive(Debug)]
pub struct InvalidBuiltin;
impl TryFrom<u8> for Builtin {
    type Error = InvalidBuiltin;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > Self::Exit as u8 {
            return Err(InvalidBuiltin);
        }
        Ok(unsafe { std::mem::transmute(value) })
    }
}
