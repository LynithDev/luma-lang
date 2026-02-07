// MARK: Diagnostics
#[macro_export]
macro_rules! define_diagnostics {
    (
        $enum_vis:vis enum $enum_name:ident {
            $(
                #[$level:tt($title:expr $(, $annotation:expr)?)]
                $err_name:ident $( { $($field:ident : $typ:ty),* $(,)? } )?
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        $enum_vis enum $enum_name {
            $(
                $err_name $( { $($field : $typ),* } )?,
            )*
        }

        impl $crate::AsDiagnostic for $enum_name {
            fn level(&self) -> $crate::DiagnosticLevel {
                match self {
                    $(
                        $enum_name::$err_name $( { $($field),* } )? => $crate::DiagnosticLevel::$level,
                    )*
                }
            }

            fn title(&self) -> String {
                match self {
                    $(
                        $enum_name::$err_name $( { $($field),* } )? => format!($title),
                    )*
                }
            }

            fn annotation(&self) -> Option<String> {
                match self {
                    $(
                        $enum_name::$err_name $( { $($field),* } )? => $crate::define_diagnostics!(@opt $($annotation)?),
                    )*
                }
            }
        }
    };

    // annotation supplied
    (@opt $annotation:expr) => { Some(format!($annotation)) };

    // annotation not supplied
    (@opt) => { None };
}

#[macro_export]
macro_rules! diagnostic {
    ($level:expr, $error:expr, [$($contexts:expr,)*$(,)?], $span:expr$(,)?) => {
        $crate::Diagnostic {
            level: $level,
            title: $crate::AsDiagnostic::title(&$error),
            annotation: $crate::AsDiagnostic::annotation(&$error),
            span: $span,
            additional_contexts: vec![
                $(
                    $contexts,
                )*
            ],

            #[cfg(debug_assertions)]
            thrower: $crate::CallerInfo {
                file: file!(),
                line: line!(),
                column: column!(),
            },
        }
    };
}

#[macro_export]
macro_rules! error {
    ($error:expr, [$($contexts:expr),* $(,)?], $span:expr$(,)?) => {
        $crate::diagnostic!($crate::DiagnosticLevel::Error, $error, [$($contexts,)*], Some($span))
    };

    ($error:expr, [$($contexts:expr),* $(,)?]$(,)?) => {
        $crate::diagnostic!($crate::DiagnosticLevel::Error, $error, [$($contexts,)*], None)
    };

    ($error:expr, $span:expr$(,)?) => {
        $crate::error!($error, [], $span)
    };

    ($error:expr$(,)?) => {
        $crate::error!($error, [])
    };
}

#[macro_export]
macro_rules! warning {
    ($error:expr, [$($contexts:expr),* $(,)?], $span:expr$(,)?) => {
        $crate::diagnostic!($crate::DiagnosticLevel::Warning, $error, [$($contexts,)*], Some($span))
    };

    ($error:expr, [$($contexts:expr),* $(,)?]$(,)?) => {
        $crate::diagnostic!($crate::DiagnosticLevel::Warning, $error, [$($contexts,)*], None)
    };

    ($error:expr, $span:expr$(,)?) => {
        $crate::warning!($error, [], $span)
    };

    ($error:expr$(,)?) => {
        $crate::warning!($error, [])
    };
}

#[macro_export]
macro_rules! note {
    ($error:expr, [$($contexts:expr),* $(,)?], $span:expr$(,)?) => {
        $crate::diagnostic!($crate::DiagnosticLevel::Note, $error, [$($contexts,)*], Some($span))
    };

    ($error:expr, [$($contexts:expr),* $(,)?]$(,)?) => {
        $crate::diagnostic!($crate::DiagnosticLevel::Note, $error, [$($contexts,)*], None)
    };

    ($error:expr, $span:expr$(,)?) => {
        $crate::note!($error, [], $span)
    };

    ($error:expr$(,)?) => {
        $crate::note!($error, [])
    };
}

// MARK: Contexts
#[macro_export]
macro_rules! define_contexts {
    (
        $enum_vis:vis enum $enum_name:ident {
            $(
                #[$kind:tt$(($annotation:expr))?]
                $err_name:ident $( { $($field:ident : $typ:ty),* $(,)? } )?
            ),* $(,)?
        }
    ) => {
        #[derive(Debug, Clone)]
        $enum_vis enum $enum_name {
            $(
                $err_name $( { $($field : $typ),* } )?,
            )*
        }

        impl $crate::AsDiagnosticContext for $enum_name {
            fn kind(&self) -> $crate::DiagnosticContextKind {
                match self {
                    $(
                        $enum_name::$err_name $( { $($field),* } )? => $crate::DiagnosticContextKind::$kind,
                    )*
                }
            }

            fn annotation(&self) -> Option<String> {
                match self {
                    $(
                        $enum_name::$err_name $( { $($field),* } )? => $crate::define_contexts!(@opt $($annotation)?),
                    )*
                }
            }
        }
    };

    (@opt $annotation:expr) => {
        Some(format!($annotation))
    };

    (@opt) => { None };
}

#[macro_export]
macro_rules! context {
    ($ctx:expr, $span:expr$(,)?) => {
        $crate::DiagnosticContext {
            kind: $crate::AsDiagnosticContext::kind(&$ctx),
            annotation: $crate::AsDiagnosticContext::annotation(&$ctx),
            span: Some($span),
        }
    };

    ($ctx:expr$(,)?) => {
        $crate::DiagnosticContext {
            kind: $crate::AsDiagnosticContext::kind(&$ctx),
            annotation: $crate::AsDiagnosticContext::annotation(&$ctx),
            span: None,
        }
    };
}
