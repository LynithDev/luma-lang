use luma_compiler::LumaCompiler;
use luma_core::CodeSource;
use luma_diagnostic::Printer;

fn main() {
    let sources = vec![
        CodeSource::new(include_str!("../../../examples/sample.luma").to_string(), Some("examples/sample.luma".to_string())),
    ];

    let compiler = LumaCompiler::new();
    let result = compiler.compile(sources);

    if result.diagnostics.is_empty() {
        println!("Compilation successful!");
    } else {
        let output = Printer::print(&result.sources, &result.diagnostics);
        println!("{}", output);
    }
}