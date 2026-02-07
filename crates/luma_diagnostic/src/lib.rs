#![feature(error_generic_member_access)]
#![feature(cfg_select)]

cfg_select! {
    feature = "pretty" => {
        mod pretty_print;
        pub use pretty_print::Printer;
    }
}

mod macros;
mod diagnostic;
pub use diagnostic::*;

pub type CompilerResult<T> = Result<T, Diagnostic>;


// pub enum CompilerContext {
//     #[context("while trying to assign to variable '{name}'")]
//     Assignment {
//         name: String,
//     }
// }

// add_error(CompilerError::MismatchedTypes {
//     expected: "i32".to_string(),
//     found: "f64".to_string(),
// }, vec![
//     CompilerContext::Assignment {
//         name: "x".to_string(),
//     }
// ]);