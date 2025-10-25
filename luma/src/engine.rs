use luma_core::{ast::Ast, CodeSource};
use luma_diagnostic::{DiagnosticResult, Reporter};
use luma_lexer::LumaLexer;
use luma_parser::LumaParser;
use luma_semantics::{LumaAnalyzer, ParsedCodeKind, ParsedCodeSource};

// #[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u8)]
// pub enum Verbosity {
//     Quiet = 0, // doesn't show any output
//     #[default]
//     Normal = 1, // shows basic output
//     Verbose = 2, // todo: implement verbose logging
// }

pub struct LumaEngine {
    reporter: Reporter,
    // verbosity: Verbosity,
}

impl LumaEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            reporter: Reporter::new(),
            // verbosity: Verbosity::Normal,
        }
    }

    pub fn eval_sources(&self, sources: Vec<CodeSource>) -> DiagnosticResult<i32> {
        let mut parsed: Vec<ParsedCodeSource> = Vec::new();

        let mut analyzer = LumaAnalyzer::new(&self.reporter);
        for source in sources {
            let ast = self.parse_ast(&source)?;
            parsed.push(ParsedCodeSource::new(source, ParsedCodeKind::Ast(ast)));
        }

        analyzer.add_entries(&parsed);
        analyzer.analyze();

        for source in &parsed {
            print!("{}", self.reporter.formatted_for(&source.source))
        }
        
        Ok(0)
    }

    pub fn eval_str(&self, _src: &str) -> DiagnosticResult<&Reporter> {
        // let input = CodeInput::from(src);

        // let ast = self.parse_ast(&input)?;
        // let input = input.with_ast(ast);

        Ok(&self.reporter)
    }

    pub fn reporter(&self) -> &Reporter {
        &self.reporter
    }

    fn parse_ast(&self, input: &CodeSource) -> DiagnosticResult<Ast> {
        let reporter = self.reporter.with_name(&input.source_name());
        let src = input.source().as_bytes();

        let mut lexer = LumaLexer::new(src, &reporter);
        let mut token_stream = lexer.scan();

        let mut parser = LumaParser::new(&mut token_stream, &reporter);
        let ast = parser.parse();

        Ok(ast)
    }

}