mod macros {
    macro_rules! define_options {
        (
            $struct_vis:vis struct $struct_name:ident {
                $($field_ident:ident:$field_ty:ty$(=$field_value:expr)?,)*
            }
        ) => {
            #[derive(Default, Debug)]
            #[non_exhaustive]
            $struct_vis struct $struct_name {
                $(pub $field_ident: $field_ty,)*
            }

            impl $struct_name {
                pub fn new() -> Self {
                    Self {
                        $($field_ident: define_options!(@default $field_ty, $($field_value)?),)*
                    }
                }
            }
        };

        (@default $field_ty:ty, $field_value:expr) => {
            $field_value
        };

        (@default $field_ty:ty,) => {
            Default::default()
        };
    }
    
    pub(crate) use define_options;
}

pub(crate) use macros::define_options;

use crate::stages::lexer::LexerOptions;

define_options! {
    pub struct CompilerOptions {
        lexer: LexerOptions,
    }
}


