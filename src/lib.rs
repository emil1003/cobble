pub mod assembler;
pub mod compiler;
pub mod interpreter;

#[test]
fn test_integration() {
    // Private file loader
    fn load_from_file(path: &str) -> Result<String, String> {
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read file {}: {}", path, e))
    }

    // Load example file
    let path = "examples/fib.asm";
    let src = load_from_file(&path).expect("examples/fib.asm should load correctly");

    // Compile program
    let prg = compiler::compile_program(&src).expect("examples/fib.asm should compile correctly");

    // Interpret program
    let (res, state) = interpreter::interpret_program(prg, None);
    res.expect("examples/fib.asm should interpret correctly");

    // Check final state
    assert_eq!(state.regs.r(3).unwrap(), 8);
}
