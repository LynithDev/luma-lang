#![feature(error_generic_member_access)]
#![feature(cfg_select)]

cfg_select! {
    feature = "pretty" => {
        mod pretty_print;
        pub use pretty_print::Printer;
    }
}

mod error;
pub use error::{LumaError, ErrorSource};

pub type CompilerResult<T> = Result<T, LumaError>;

