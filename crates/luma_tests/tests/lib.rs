mod parsing;
mod exec;

pub use helpers::*;

mod helpers {
    use luma::LumaEngine;
    use luma_core::{ast::Ast, CodeSource};
    use luma_diagnostic::Reporter;
    use luma_lexer::{tokens::TokenStream, LumaLexer};
    use luma_parser::LumaParser;
    use luma_semantics::{LumaAnalyzer, ParsedCodeKind, ParsedCodeSource};
    use luma_vm::{runtime::RuntimeOptions, LumaVM, ProgramSource, VmHandle};

    pub fn tokenize_input(reporter: &Reporter, input: &str) -> TokenStream {
        let mut lexer = LumaLexer::new(input.as_bytes(), reporter);
        lexer.scan()
    }

    pub fn parse_tokens(reporter: &Reporter, tokens: &mut TokenStream) -> Ast {
        let mut parser = LumaParser::new(tokens, reporter);
        parser.parse()
    }

    pub fn parse_source(
        reporter: &Reporter,
        input: &str,
    ) -> ParsedCodeSource {
        let code_source = CodeSource::from(input);

        let mut tokens = tokenize_input(reporter, code_source.source());
        let ast = parse_tokens(reporter, &mut tokens);
        ParsedCodeSource::new(code_source, ParsedCodeKind::Ast(ast))
    }

    pub fn analyze_source(
        reporter: &Reporter,
        code_source: &ParsedCodeSource,
    ) -> bool {
        let mut semantics_analyzer = LumaAnalyzer::new(reporter);
        semantics_analyzer.add_entry(code_source);
        semantics_analyzer.analyze()
    }

    pub fn compile(input: &str) -> Vec<ProgramSource> {
        let engine = LumaEngine::new();
        engine.compile(vec![CodeSource::from(input)]).expect("couldn't compile input")
    }

    pub fn initialize_vm(sources: Vec<ProgramSource>) -> VmHandle {
        initialize_vm_opts(sources, RuntimeOptions::default())
    }

    pub fn initialize_vm_opts(sources: Vec<ProgramSource>, options: RuntimeOptions) -> VmHandle {
        LumaVM::with_options(sources, options).expect("failed to create VM")
    }
}
