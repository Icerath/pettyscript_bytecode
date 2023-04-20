use pettyscript_bytecode::assembler::assemble;
use pettyscript_bytecode::vm;

fn main() {
    let content = std::fs::read_to_string("example.pty").unwrap();
    let program = assemble(&content);
    println!("{program}");
    let stack = vm::create_and_run(&program);
    if !stack.is_empty() {
        println!("{stack:?}");
    }
}
