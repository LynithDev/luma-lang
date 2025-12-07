use luma_codegen::LumaCodegen;
use luma_core::{ast::Ast, CodeSource};
use luma_diagnostic::{DiagnosticKind, Reporter};
use luma_lexer::LumaLexer;
use luma_parser::LumaParser;
use luma_semantics::{LumaAnalyzer, ParsedCodeKind, ParsedCodeSource};
use luma_vm::{LumaVM, ProgramSource, VmExitResult};

pub struct LumaEngine {
    reporter: Reporter,
}

impl LumaEngine {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            reporter: Reporter::new(),
        }
    }

    /// Evaluate multiple code sources. The first source is treated as the main entry point.
    pub fn eval_sources(&self, sources: Vec<CodeSource>) -> VmExitResult {
        let program_sources = match self.compile(sources) {
            Ok(sources) => sources,
            Err(code) => return VmExitResult::from_code(code),
        };

        // vm
        let mut vm = match LumaVM::try_new(program_sources) {
            Ok(vm) => vm,
            Err(e) => {
                eprintln!("VM Initialization Error: {}", e);
                return VmExitResult::from_error(e);
            }
        };

        vm.run()
    }

    /// Evaluate a single code source provided as a string.
    pub fn eval_str(&self, src: &str) -> VmExitResult {
        let code_source = CodeSource::from(src);

        self.eval_sources(vec![code_source])
    }

    /// Get a reference to the reporter.
    pub fn reporter(&self) -> &Reporter {
        &self.reporter
    }

    /// Compile multiple code sources into program sources. Program sources contain 
    /// the necessary information to be run by the VM.
    pub fn compile(&self, sources: Vec<CodeSource>) -> Result<Vec<ProgramSource>, i32> {
        // parsing
        let mut parsed: Vec<ParsedCodeSource> = Vec::new();
        for source in sources {
            let ast = self.parse_ast(&source);
            parsed.push(ParsedCodeSource::new(source, ParsedCodeKind::Ast(ast)));
        }
        
        // analysis
        let mut analyzer = LumaAnalyzer::new(&self.reporter);
        analyzer.add_entries(&parsed);
        analyzer.analyze();
        self.report_diagnostics(&parsed, i32::MIN)?;

        // codegen
        let mut codegen = LumaCodegen::new(&self.reporter);
        codegen.add_entries(&parsed);
        codegen.generate();
        self.report_diagnostics(&parsed, i32::MIN + 1)?;

        // collect bytecode
        let mut program_sources: Vec<ProgramSource> = Vec::new();
        for p in parsed {
            let code = p.code.into_inner();
            
            let ParsedCodeKind::Bytecode(bytecode) = code else {
                eprintln!("Internal Error: Codegen did not produce bytecode");
                continue;
            };
            
            program_sources.push(ProgramSource::new(p.source.kind().to_owned(), bytecode));
        }

        Ok(program_sources)
    }

    fn report_diagnostics(&self, sources: &Vec<ParsedCodeSource>, exit_code: i32) -> Result<(), i32> {
        let errored = self.reporter.diagnostic_count(DiagnosticKind::Error) > 0;
        for source in sources {
            print!("{}", self.reporter.formatted_for(&source.source))
        }

        if errored {
            Err(exit_code)
        } else {
            Ok(())
        }
    }

    fn parse_ast(&self, input: &CodeSource) -> Ast {
        let reporter = self.reporter.with_name(&input.source_name());
        let src = input.source().as_bytes();

        let mut lexer = LumaLexer::new(src, &reporter);
        let mut token_stream = lexer.scan();

        let mut parser = LumaParser::new(&mut token_stream, &reporter);
        parser.parse()
    }

}