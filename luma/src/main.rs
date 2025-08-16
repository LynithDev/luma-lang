use std::{io::Write, path::PathBuf, str::FromStr};

use luma::LumaEngine;
use luma_core::CodeInput;

// const SOURCE: &str = r#"
// pub class Object {
// 	var mut name: Option<string>;
	
// 	pub construct(name: String) {
// 		this.name = name;
// 	}
	
// 	pub doSmth<E : Error>(this): Result<string, E> {
// 		Ok(if this.name.isEmpty() {
//             error
//         } else {
//             "hello " + this.name
//         })
// 	}
// }
// "#;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    if args.len() <= 1 {
        repl()
    } else {
        compile(&args)
    }
}

fn repl() {
    let engine = LumaEngine::new();

    loop {
        let mut input = String::new();

        std::io::stdout().write_all(b"> ").expect("failed to write prompt");
        std::io::stdout().flush().expect("failed to flush stdout");
        std::io::stdin().read_line(&mut input).expect("failed to read line");

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let _ = engine.eval_str(input);
    }
}

fn compile(args: &[String]) {
    let files = args.iter().skip(1).flat_map(|f| PathBuf::from_str(f)).collect::<Vec<_>>();

    let engine = LumaEngine::new();
    
    let inputs = files.into_iter()
        .flat_map(|path| CodeInput::try_from(path).ok())
        .collect::<Vec<_>>();

    let _ = engine.eval_sources(inputs);
}
