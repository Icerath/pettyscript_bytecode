use std::collections::HashMap;

use crate::{cursor::Cursor, op_codes::OpCode, program::Program};

#[must_use]
pub fn compile_str(input: &str) -> Program {
    let tokens = tokenize(input);
    compile_tokens(tokens)
}

#[must_use]
pub fn compile_tokens<'a, I: Iterator<Item = Token<'a>>>(tokens: I) -> Program {
    let mut program = Program::new();

    let mut jumps: HashMap<&str, usize> = HashMap::default();
    let mut incomplete_jumps: HashMap<&str, Vec<usize>> = HashMap::default();

    for token in tokens {
        match token {
            Token::Comment | Token::Whitespace => {}

            Token::Add => program.push_opcode(OpCode::Add),
            Token::Sub => program.push_opcode(OpCode::Sub),
            Token::Mul => program.push_opcode(OpCode::Mul),
            Token::Div => program.push_opcode(OpCode::Div),

            Token::Le => program.push_opcode(OpCode::Le),
            Token::Lt => program.push_opcode(OpCode::Lt),
            Token::Ge => program.push_opcode(OpCode::Ge),
            Token::Gt => program.push_opcode(OpCode::Gt),
            Token::Eq => program.push_opcode(OpCode::Eq),
            Token::Not => program.push_opcode(OpCode::UnaryNot),

            Token::Int(int) => _ = program.push_literal(int),
            Token::Float(float) => _ = program.push_literal(float),
            Token::Str(str) => _ = program.push_literal(str.to_owned()),

            Token::Jump(str) => {
                let location = jumps.get(str).copied().unwrap_or(0);
                let jump = program.push_jump(location);
                if jumps.get(str).is_none() {
                    incomplete_jumps.entry(str).or_default().push(jump);
                }
            }

            Token::OptJump(str) => {
                let location = jumps.get(str).copied().unwrap_or(0);
                let jump = program.push_pop_jump_if_false(location);
                if jumps.get(str).is_none() {
                    incomplete_jumps.entry(str).or_default().push(jump);
                }
            }

            Token::Flag(str) => {
                assert!(!jumps.contains_key(str));
                jumps.insert(str, program.len());
                if let Some(to_patch) = incomplete_jumps.get(str) {
                    for jump in to_patch {
                        program.patch_jump(*jump);
                    }
                    incomplete_jumps.remove(str);
                }
            }
            Token::Keyword("ret") => program.push_opcode(OpCode::Ret),
            Token::Keyword("pop") => program.push_opcode(OpCode::Pop),
            Token::Keyword("swap") => program.push_opcode(OpCode::Swap),
            Token::Keyword("dup") => program.push_opcode(OpCode::Dup),
            Token::Keyword("dup_swap") => program.push_opcode(OpCode::DupSwap),
            Token::Keyword(keyword) => todo!("{keyword}"),
            Token::End => unreachable!(),
        }
    }

    program
}

#[derive(Debug)]
pub enum Token<'a> {
    Whitespace,
    Comment,

    Int(i64),
    Float(f64),
    Str(&'a str),

    Add,
    Sub,
    Mul,
    Div,

    Not,

    Le,
    Lt,
    Ge,
    Gt,
    Eq,

    Keyword(&'a str),
    Flag(&'a str),
    Jump(&'a str),
    OptJump(&'a str),

    End,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || cursor.next_token())
        .filter(|tok| !matches!(tok, Token::Whitespace | Token::Comment))
}

pub fn filter_tokenize(input: &str) -> impl Iterator<Item = Token> {
    tokenize(input).filter(|token| !matches!(token, Token::Whitespace | Token::Comment))
}

impl<'a> Cursor<'a> {
    fn next_token(&mut self) -> Option<Token<'a>> {
        let ch = self.bump()?;
        let token = match ch {
            _ if ch.is_whitespace() => self.whitespace(),
            '/' if self.peek() == Some('/') => self.line_comment(),
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,

            '<' if self.peek() == Some('=') => {
                self.bump();
                Token::Le
            }
            '>' if self.peek() == Some('=') => {
                self.bump();
                Token::Ge
            }
            '<' => Token::Lt,
            '>' => Token::Gt,
            '=' => Token::Eq,
            '!' => Token::Not,

            '0'..='9' => self.parse_num(self.head - 1),
            '"' => self.parse_string(),

            '@' => Token::Flag(self.parse_ident(self.head)),
            '$' => Token::Jump(self.parse_ident(self.head)),
            '?' => Token::OptJump(self.parse_ident(self.head)),

            _ if ch.is_alphabetic() => Token::Keyword(self.parse_ident(self.head - 1)),
            _ => todo!("({ch})"),
        };
        Some(token)
    }

    fn parse_num(&mut self, start: usize) -> Token<'a> {
        self.take_while(char::is_numeric);
        if self.remaining().starts_with('.') {
            self.bump();
            self.take_while(char::is_numeric);
            let string = &self.text[start..self.head];
            let float = string.parse().unwrap();
            Token::Float(float)
        } else {
            let string = &self.text[start..self.head];
            let int = string.parse().unwrap();
            Token::Int(int)
        }
    }

    fn parse_string(&mut self) -> Token<'a> {
        let start = self.head;
        self.take_while(|c| c != '"');
        let string = &self.text[start..self.head];
        self.bump();
        Token::Str(string)
    }

    fn parse_ident(&mut self, start: usize) -> &'a str {
        self.take_while(|ch| ch.is_alphanumeric() || ch == '_');
        &self.text[start..self.head]
    }
    fn whitespace(&mut self) -> Token<'a> {
        self.take_while(|c| c.is_ascii_whitespace());
        Token::Whitespace
    }

    fn line_comment(&mut self) -> Token<'a> {
        self.take_while(|c| c != '\n');
        Token::Comment
    }
}
