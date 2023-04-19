use std::fmt;

use crate::{op_codes::OpCode, program::Program};

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut head = 0;
        while head < self.len() {
            let op_code = OpCode::try_from(self[head]).unwrap();
            head += 1;

            write!(f, "{op_code:?}")?;
            match op_code {
                OpCode::LoadConst => {
                    let index_bytes = self.read_arr(head).unwrap();
                    let index = u32::from_le_bytes(index_bytes) as usize;
                    let constant = self.constants[index].clone();
                    write!(f, " {index} {constant}")?;
                }
                OpCode::Jump | OpCode::PopJumpIfFalse => {
                    let index_bytes = self.read_arr(head).unwrap();
                    let index = u32::from_le_bytes(index_bytes) as usize;
                    write!(f, " {index}")?;
                }
                _ => (),
            }
            writeln!(f)?;
            head += op_code.size_operand();
        }
        Ok(())
    }
}

#[cfg(test)]
#[test]
fn test_dis() {
    let mut program = Program::new();

    program.push_opcode(OpCode::NOP);
    program.push_literal("Hello");
    program.push_literal(3);
    program.push_opcode(OpCode::Mul);

    program.push_opcode(OpCode::Dup);

    let dis = program.to_string();
    assert_eq!(
        dis,
        r#"NOP
LoadConst 0 'Hello'
LoadConst 1 3
Mul
Dup
"#
    );
}
