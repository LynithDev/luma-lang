use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, LitStr};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = &input.ident;

    // Ensure we have an enum
    let data_enum = match &input.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "DiagnosticMessage can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    let mut kind_arms = Vec::new();
    let mut note_arms = Vec::new();
    let mut display_arms = Vec::new();

    for variant in &data_enum.variants {
        let v_ident = &variant.ident;

        // Fix pattern and args generation
        let (pat_tokens, fmt_args) = match &variant.fields {
            Fields::Unit => (quote! {}, vec![]),
            Fields::Unnamed(u) if u.unnamed.is_empty() => (quote! {}, vec![]),
            Fields::Unnamed(u) => {
                let idents: Vec<_> = (0..u.unnamed.len())
                    .map(|i| syn::Ident::new(&format!("_f{i}"), v_ident.span()))
                    .collect();
                let pat = quote! { ( #(#idents),* ) };
                let args = idents.iter().map(|id| quote! { #id }).collect();
                (pat, args)
            }
            Fields::Named(n) => {
                let idents: Vec<_> = n
                    .named
                    .iter()
                    .map(|f| f.ident.clone().expect("named field must have ident"))
                    .collect();
                let pat = quote! { { #(#idents),* } };
                let args = idents.iter().map(|id| quote! { #id }).collect();
                (pat, args)
            }
        };

        // Parse attributes as before...
        let mut found_kind: Option<proc_macro2::TokenStream> = None;
        let mut message_template: Option<String> = None;
        let mut note_text: Option<String> = None;

        for attr in &variant.attrs {
            if attr.path().is_ident("warning") || attr.path().is_ident("error") {
                let kind_token = if attr.path().is_ident("warning") {
                    quote! { ::luma_diagnostic::DiagnosticKind::Warning }
                } else {
                    quote! { ::luma_diagnostic::DiagnosticKind::Error }
                };
                found_kind = Some(kind_token);

                if let Ok(litstr) = attr.parse_args::<LitStr>() {
                    message_template = Some(litstr.value());
                }
            } else if attr.path().is_ident("note")
                && let Ok(litstr) = attr.parse_args::<LitStr>() {
                    note_text = Some(litstr.value());
                }
        }

        let kind_tokens = if let Some(k) = found_kind {
            k
        } else {
            return syn::Error::new_spanned(
                v_ident,
                "each variant must have either #[warning(\"...\")] or #[error(\"...\")]",
            )
            .to_compile_error()
            .into();
        };

        kind_arms.push(quote! {
            Self::#v_ident #pat_tokens => #kind_tokens
        });

        if let Some(note_s) = note_text {
            note_arms.push(quote! {
                Self::#v_ident #pat_tokens => Some(#note_s.to_string())
            });
        }

        if let Some(template) = message_template {
            let mut fmt = template.replace("{{", "{{{{").replace("}}", "}}}}");
            for i in 0..fmt.matches('{').count() + 1 {
                let token = format!("{{{i}}}");
                if fmt.contains(&token) {
                    fmt = fmt.replacen(&token, "{}", 1);
                }
            }

            let args = &fmt_args;
            if args.is_empty() {
                display_arms.push(quote! {
                    Self::#v_ident => {
                        write!(f, #fmt)
                    }
                });
            } else {
                display_arms.push(quote! {
                    Self::#v_ident #pat_tokens => {
                        write!(f, #fmt, #(#args),*)
                    }
                });
            }
        } else {
            display_arms.push(quote! {
                Self::#v_ident #pat_tokens => {
                    write!(f, "{:?}", self)
                }
            });
        }
    }

    let note_fn = if note_arms.is_empty() {
        quote! {
            fn note(&self) -> Option<String> {
                None
            }
        }
    } else {
        quote! {
            fn note(&self) -> Option<String> {
                match self {
                    #(#note_arms),*,
                    _ => None
                }
            }
        }
    };

    let expanded = quote! {
        impl ::luma_diagnostic::DiagnosticMessage for #enum_ident {
            fn kind(&self) -> ::luma_diagnostic::DiagnosticKind {
                match self {
                    #(#kind_arms),*
                }
            }

            #note_fn
        }

        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#display_arms),*
                }
            }
        }
    };

    expanded.into()
}