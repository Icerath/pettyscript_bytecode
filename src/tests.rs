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

#[test]
fn test_pop_jump_if_false() {
    let mut program = Program::new();

    program.push_literal(0);
    let jump = program.push_pop_jump_if_false(0);

    program.push_literal(1);
    program.push_literal(2);

    program.patch_jump(jump);
    program.push_literal(3);

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::Int(3)]);
}

#[test]
fn test_if_true() {
    let mut program = Program::new();

    program.push_literal(1);
    program.push_if(|body| {
        body.push_literal("Hello, ");
    });
    program.push_literal("World!");

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::from("Hello, "), Value::from("World!")]);
}

#[test]
fn test_if_false() {
    let mut program = Program::new();

    program.push_literal(0);
    program.push_if(|body| {
        body.push_literal("Hello, ");
    });
    program.push_literal("World!");

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::from("World!")]);
}

#[test]
fn test_if_else_true() {
    let mut program = Program::new();

    program.push_literal(1);
    program.push_if_or_else(
        |body| {
            body.push_literal("Hello, ");
        },
        |or_else| {
            or_else.push_literal("Goodbye, ");
        },
    );
    program.push_literal("World!");

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::from("Hello, "), Value::from("World!")]);
}

#[test]
fn test_if_else_false() {
    let mut program = Program::new();

    program.push_literal(0);
    program.push_if_or_else(
        |body| {
            body.push_literal("Hello, ");
        },
        |or_else| {
            or_else.push_literal("Goodbye, ");
        },
    );
    program.push_literal("World!");

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::from("Goodbye, "), Value::from("World!")]);
}

#[test]
fn test_while_loop() {
    let mut program = Program::new();

    program.push_literal(0);

    program.push_while_loop(
        |condition| {
            condition.push_opcode(OpCode::Dup);
            condition.push_literal(10);
            condition.push_opcode(OpCode::Lt);
        },
        |body| {
            body.push_literal(1);
            body.push_opcode(OpCode::Add);
        },
    );

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::Int(10)]);
}

#[test]
fn while_range_product() {
    let mut program = Program::new();

    //product = 1
    program.push_literal(1);
    // i = 0
    program.push_literal(0);
    program.push_while_loop(
        // i < 10
        |condition| {
            condition.push_opcode(OpCode::Dup);
            condition.push_literal(5);
            condition.push_opcode(OpCode::Lt);
        },
        |body| {
            // i += 1
            body.push_literal(1);
            body.push_opcode(OpCode::Add);
            // product *= i
            {
                body.push_opcode(OpCode::DupSwap);
                body.push_opcode(OpCode::Mul);
                body.push_opcode(OpCode::Swap);
            }
        },
    );

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::Int(5 * 4 * 3 * 2), Value::Int(5)]);
}

#[test]
fn test_load_store_name() {
    let mut program = Program::new();

    program.push_literal(1);
    program.store_name("x");
    program.load_name("x");
    program.load_name("x");

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::Int(1), Value::Int(1)]);
}

#[test]
fn test_func_call() {
    let mut program = Program::new();

    let func = program.push_func(|func| {
        func.push_literal(1);
        func.push_opcode(OpCode::Add);
    });

    program.push_literal(3);
    program.call_func(func);
    program.push_literal(2);

    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    assert_eq!(stack, vec![Value::Int(4), Value::Int(2)]);
}
