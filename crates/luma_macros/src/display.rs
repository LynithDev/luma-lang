use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let enum_ident = &input.ident;

    let data_enum = match &input.data {
        Data::Enum(e) => e,
        _ => {
            return syn::Error::new_spanned(
                &input.ident,
                "SimpleDisplay can only be derived for enums",
            )
            .to_compile_error()
            .into();
        }
    };

    // Check enum-level attributes for global case style
    let mut case_style: Option<String> = None;
    for attr in &input.attrs {
        if attr.path().is_ident("display") {
            let result = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("case") {
                    case_style = Some(meta.value()?.parse::<syn::LitStr>()?.value());

                    return Ok(());
                }

                Err(meta.error("expected `case = \"style\"`"))
            });

            if let Err(err) = result {
                return err.to_compile_error().into();
            }
        }
    }

    let mut arms = Vec::new();

    for variant in &data_enum.variants {
        let v_ident = &variant.ident;
        let mut display_str: Option<String> = None;

        for attr in &variant.attrs {
            if attr.path().is_ident("display")
                && let Ok(litstr) = attr.parse_args::<syn::LitStr>() {
                    display_str = Some(litstr.value());
                }
        }

        let final_str = if let Some(s) = display_str {
            s
        } else if let Some(ref case) = case_style {
            convert_case(&v_ident.to_string(), case)
        } else {
            v_ident.to_string()
        };

        match &variant.fields {
            Fields::Unit => {
                arms.push(quote! {
                    Self::#v_ident => f.write_str(#final_str)
                });
            }
            Fields::Unnamed(_) => {
                arms.push(quote! {
                    Self::#v_ident(..) => f.write_str(#final_str)
                });
            }
            Fields::Named(_) => {
                arms.push(quote! {
                    Self::#v_ident { .. } => f.write_str(#final_str)
                });
            }
        }
    }

    let expanded = quote! {
        impl std::fmt::Display for #enum_ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#arms),*
                }
            }
        }
    };

    expanded.into()
}

fn convert_case(name: &str, case: &str) -> String {
    match case {
        "lowercase" => {
            name.to_lowercase()
        }
        "snake_case" => {
            let mut out = String::new();
            for (i, ch) in name.chars().enumerate() {
                if ch.is_uppercase() {
                    if i != 0 {
                        out.push('_');
                    }
                    out.extend(ch.to_lowercase());
                } else {
                    out.push(ch);
                }
            }
            out
        }
        "kebab-case" => {
            let mut out = String::new();
            for (i, ch) in name.chars().enumerate() {
                if ch.is_uppercase() {
                    if i != 0 {
                        out.push('-');
                    }
                    out.extend(ch.to_lowercase());
                } else {
                    out.push(ch);
                }
            }
            out
        }
        "SCREAMING_SNAKE_CASE" => {
            let mut out = String::new();
            for (i, ch) in name.chars().enumerate() {
                if ch.is_uppercase() {
                    if i != 0 {
                        out.push('_');
                    }
                    out.push(ch);
                } else {
                    out.push(ch.to_ascii_uppercase());
                }
            }
            out
        }
        _ => name.to_string(),
    }
}
