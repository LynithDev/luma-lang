use luma_compiler::LumaCompiler;
use luma_core::CodeSource;

fn main() {
    let sources = vec![
        CodeSource::from(include_str!("../../../examples/sample.luma")),
    ];

    let compiler = LumaCompiler::new();
    let result = compiler.compile(sources);

    if !result.errors.is_empty() {
        eprintln!("Compilation failed with the following errors:");
        
        for error in result.errors {
            eprintln!("- {}", error);
        }

    } else {
        println!("Compilation succeeded without errors.");
    }
}