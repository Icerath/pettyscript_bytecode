use pettyscript_bytecode::assembler::compile_str;
use pettyscript_bytecode::value::Value;
use pettyscript_bytecode::vm;

fn main() {
    let content = std::fs::read_to_string("examples/while_loop.pty").unwrap();
    let program = compile_str(&content);
    eprintln!("{program}");
    let stack = vm::create_and_run(&program);
    if !stack.is_empty() {
        print_stack(&stack);
    }
}

fn print_stack(stack: &[Value]) {
    println!("Remaining stack:");
    for value in stack {
        println!("| {value},");
    }
}
