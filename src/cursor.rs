use std::str::Chars;

pub(crate) struct Cursor<'a> {
    pub head: usize,
    pub chars: Chars<'a>,
    pub text: &'a str,
}

impl<'a> Cursor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            head: 0,
            chars: input.chars(),
            text: input,
        }
    }
    pub fn bump(&mut self) -> Option<char> {
        let ch = self.chars.next()?;
        self.head += ch.len_utf8();
        Some(ch)
    }
    pub fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }
    pub fn remaining(&self) -> &'a str {
        self.chars.as_str()
    }
    pub fn take_while<F>(&mut self, mut predicate: F)
    where
        F: FnMut(char) -> bool,
    {
        while let Some(ch) = self.peek() {
            if !predicate(ch) {
                break;
            }
            self.bump();
        }
    }
}
