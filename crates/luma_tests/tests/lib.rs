mod parsing;

pub use helpers::*;

mod helpers {
    use luma_core::{ast::Ast, CodeSource};
    use luma_diagnostic::Reporter;
    use luma_lexer::{tokens::TokenStream, LumaLexer};
    use luma_parser::LumaParser;
    use luma_semantics::{LumaAnalyzer, ParsedCodeKind, ParsedCodeSource};

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
}
