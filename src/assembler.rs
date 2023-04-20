use std::collections::HashMap;

use crate::{builtins::Builtin, cursor::Cursor, op_codes::OpCode, program::Program};

#[must_use]
pub fn assemble(input: &str) -> Program {
    let tokens = tokenize(input);
    let mut program = Program::new();
    let mut functions: HashMap<&str, usize> = HashMap::default();

    load_builtins(&mut program, &mut functions);

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

            Token::Ident(name) => program.call_func(functions[name]),
            Token::DefFunc(_) => todo!(),
            Token::End => unreachable!(),
        }
    }

    program
}

fn load_builtins(program: &mut Program, functions: &mut HashMap<&str, usize>) {
    let jump_end = program.push_jump(0);
    let builtins = [("print", Builtin::Print)];
    for (name, variant) in builtins {
        let func = program.push_builtin(variant);
        program.push_opcode(OpCode::Ret);
        functions.insert(name, func);
    }
    program.patch_jump(jump_end);
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

    Ident(&'a str),
    DefFunc(&'a str),
    End,
}

pub fn tokenize(input: &str) -> impl Iterator<Item = Token> {
    let mut cursor = Cursor::new(input);
    std::iter::from_fn(move || cursor.next_token())
}

pub fn filter_tokenize(input: &str) -> impl Iterator<Item = Token> {
    tokenize(input).filter(|token| !matches!(token, Token::Whitespace | Token::Comment))
}

impl<'a> Cursor<'a> {
    fn next_token(&mut self) -> Option<Token<'a>> {
        let start = self.head;
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

            '0'..='9' => self.parse_num(start),
            '"' => self.parse_string(),
            _ if ch.is_alphabetic() => self.parse_ident(start),
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

    fn parse_ident(&mut self, start: usize) -> Token<'a> {
        self.take_while(char::is_alphanumeric);
        Token::Ident(&self.text[start..self.head])
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
