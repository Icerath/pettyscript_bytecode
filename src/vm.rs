use crate::{op_codes::OpCode, program::Program, value::Value};
use std::{
    collections::HashMap,
    ops::{Add, Div, Mul, Sub},
};

pub struct Vm<'a> {
    bytes: &'a [u8],
    constants: &'a [Value],
    stack: Vec<Value>,
    idents: &'a [String],
    variables: HashMap<&'a str, Value>,
    call_stack: Vec<usize>,
    head: usize,
}

#[allow(clippy::must_use_candidate)]
pub fn create_and_run(program: &Program) -> Vec<Value> {
    let mut vm = Vm::from(program);
    vm.run();
    vm.stack
}

impl<'a> From<&'a Program> for Vm<'a> {
    fn from(value: &'a Program) -> Self {
        Self {
            bytes: &value.bytes,
            constants: &value.constants,
            idents: &value.idents,

            variables: HashMap::default(),
            stack: vec![],
            call_stack: vec![],
            head: 0,
        }
    }
}

impl<'a> Vm<'a> {
    pub fn run(&mut self) {
        while self.head < self.bytes.len() {
            self.run_next();
        }
    }
    pub fn run_next(&mut self) {
        let op_code = OpCode::try_from(self.bytes[self.head]).unwrap();
        self.head += 1;

        match op_code {
            OpCode::Nop => {}
            OpCode::Dup => {
                let top = self.stack.last().unwrap().clone();
                self.stack.push(top);
            }
            OpCode::Pop => {
                self.stack.pop();
            }
            OpCode::Add => self.binop(Value::add),
            OpCode::Sub => self.binop(Value::sub),
            OpCode::Mul => self.binop(Value::mul),
            OpCode::Div => self.binop(Value::div),

            OpCode::Le => self.binop(cmp(Value::le)),
            OpCode::Lt => self.binop(cmp(Value::lt)),
            OpCode::Ge => self.binop(cmp(Value::ge)),
            OpCode::Gt => self.binop(cmp(Value::gt)),
            OpCode::Eq => self.binop(cmp(Value::eq)),
            OpCode::Ne => self.binop(cmp(Value::ne)),

            OpCode::LoadConst => {
                let index = self.read_u32() as usize;
                let value = self.constants[index].clone();
                self.stack.push(value);
            }
            OpCode::Jump => return self.head = self.read_u32() as usize,
            OpCode::PopJumpIfFalse => {
                let should_jump = !bool::try_from(&self.pop_stack()).unwrap();
                if should_jump {
                    return self.head = self.read_u32() as usize;
                }
            }
            OpCode::Ret => self.head = self.call_stack.pop().unwrap(),
            OpCode::PrepareFuncCall => self.call_stack.push(self.head + 5),
            OpCode::StoreName => {
                let top = self.pop_stack();

                let index = self.read_u32() as usize;
                let ident: &'a str = &self.idents[index];
                self.variables.insert(ident, top);
            }
            OpCode::LoadName => {
                let index = self.read_u32() as usize;
                let ident: &str = &self.idents[index];
                let val = self.variables.get(ident).unwrap().clone();
                self.stack.push(val);
            }
            OpCode::StopCode => unreachable!(),
        }

        self.head += op_code.size_operand();
    }
    fn pop_stack(&mut self) -> Value {
        self.stack.pop().expect("Failed to pop from stack.")
    }
    fn binop<F>(&mut self, func: F)
    where
        F: FnOnce(Value, Value) -> Value,
    {
        let rhs = self.pop_stack();
        let lhs = self.pop_stack();
        self.stack.push(func(lhs, rhs));
    }
    fn read_u32(&self) -> u32 {
        u32::from_le_bytes(self.read_arr())
    }
    fn read_arr<const LEN: usize>(&self) -> [u8; LEN] {
        let slice = &self.bytes[self.head..self.head + LEN];
        slice.try_into().unwrap()
    }
}

///
fn cmp<F>(func: F) -> impl FnOnce(Value, Value) -> Value
where
    F: FnOnce(&Value, &Value) -> bool,
{
    |lhs, rhs| Value::from(func(&lhs, &rhs))
}
