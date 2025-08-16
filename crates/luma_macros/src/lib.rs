#[cfg(feature = "diagnostic")]
mod diagnostic;

#[cfg(feature = "diagnostic")]
#[proc_macro_derive(Diagnostic, attributes(warning, error, note))]
pub fn derive_diagnostic_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    diagnostic::derive(input)
}

mod display;

#[proc_macro_derive(Display, attributes(display))]
pub fn derive_display(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    display::derive(input)
}