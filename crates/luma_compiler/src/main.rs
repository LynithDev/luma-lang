use luma_compiler::LumaCompiler;
use luma_core::CodeSource;

fn main() {
    let sources = vec![
        CodeSource::from(include_str!("../../../examples/sample.luma")),
    ];

    let compiler = LumaCompiler::new();
    let success = compiler.compile(&sources);

    if success {
        println!("Compilation succeeded!");
    } else {
        println!("Compilation failed with errors during '{}':", compiler.current_stage());
        for error in compiler.errors().iter() {
            println!("{}", error);
        }
    }
}