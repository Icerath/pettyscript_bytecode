use crate::{op_codes::OpCode, program::Program, value::Value, vm};

#[test]
fn test_binary_expressions() {
    let mut program = Program::new();
    program.push_literal(1);
    program.push_literal(2);
    program.push_opcode(OpCode::Add);

    program.push_literal(3);
    program.push_literal(4);
    program.push_opcode(OpCode::Sub);

    program.push_opcode(OpCode::Gt);

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::Int(1)]);
}

#[test]
fn test_jump() {
    let mut program = Program::new();

    let jump = program.push_jump(0);

    program.push_literal(1);
    program.push_literal(2);

    program.patch_jump(jump);
    program.push_literal(3);

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::Int(3)]);
}
