use crate::{Type, TypeKind, Visibility, VisibilityKind};
use luma_diagnostic::{CompilerResult, LumaError};

use crate::stages::{
    lexer::TokenKind,
    parser::{error::ParserErrorKind, parse::TokenParser},
};

impl TokenParser<'_> {
    // MARK: Type
    /// Parses a type annotation (does not consume anything other than the type itself)
    pub(super) fn parse_type(&mut self) -> CompilerResult<Type> {
        let current = self.current();

        match current.kind {
            TokenKind::Ident => {
                let token = self.consume(TokenKind::Ident)?;

                let type_str = token.lexeme.as_str();

                if type_str.is_empty() {
                    return Err(LumaError::spanned(
                        ParserErrorKind::InvalidType {
                            type_name: type_str.to_string(),
                        },
                        token.span,
                    ));
                }

                let kind = match type_str {
                    "u8" => TypeKind::UInt8,
                    "u16" => TypeKind::UInt16,
                    "u32" => TypeKind::UInt32,
                    "u64" => TypeKind::UInt64,
                    "usize" => {
                        if cfg!(target_pointer_width = "64") {
                            TypeKind::UInt64
                        } else {
                            TypeKind::UInt32
                        }
                    }

                    "i8" => TypeKind::Int8,
                    "i16" => TypeKind::Int16,
                    "i32" => TypeKind::Int32,
                    "i64" => TypeKind::Int64,
                    "isize" => {
                        if cfg!(target_pointer_width = "64") {
                            TypeKind::Int64
                        } else {
                            TypeKind::Int32
                        }
                    }

                    "f32" => TypeKind::Float32,
                    "f64" => TypeKind::Float64,
                    "bool" => TypeKind::Bool,
                    "char" => TypeKind::Char,
                    "str" => TypeKind::String,

                    other => TypeKind::Named {
                        name: other.to_string(),
                        def_id: None,
                    },
                };

                Ok(Type::spanned(token.span, kind))
            }

            TokenKind::Asterisk => {
                let token = self.consume(TokenKind::Asterisk)?;

                let inner_type = self.parse_type()?;

                Ok(Type::spanned(
                    token.span.maybe_merged(&inner_type.span),
                    TypeKind::Ptr(Box::new(inner_type)),
                ))
            }

            TokenKind::LeftParen => {
                let token = self.consume(TokenKind::LeftParen)?;
                let mut types = Vec::new();
                let mut span = token.span;

                while !self.check(TokenKind::RightParen) {
                    let ty = self.parse_type()?;
                    span.maybe_merge(&ty.span);
                    types.push(ty);

                    if self.consume(TokenKind::Comma).is_err() {
                        break;
                    }
                }

                let right_paren = self.consume(TokenKind::RightParen)?;
                span.merge(&right_paren.span);

                Ok(Type::spanned(
                    span,
                    match types.len() {
                        0 => TypeKind::Unit,
                        // 1 => types.into_iter().next().unwrap().item,
                        _ => TypeKind::Tuple(types),
                    },
                ))
            }

            _ => Err(LumaError::spanned(
                ParserErrorKind::InvalidType {
                    type_name: current.lexeme.clone(),
                },
                current.span,
            )),
        }
    }

    // MARK: Pub
    /// Parses the 'pub' token. It recursively calls the appropriate
    pub(super) fn parse_visibility(&mut self) -> CompilerResult<Visibility> {
        let Ok(pub_token) = self.consume(TokenKind::Pub) else {
            // not a 'pub' token, return default visibility
            return Ok(Visibility::default());
        };

        // check if it's 'pub(...)'
        if self.consume(TokenKind::LeftParen).is_ok() {
            let vis_token = self.current();

            let visibility_kind = match vis_token.kind {
                TokenKind::Module => VisibilityKind::Module,
                TokenKind::This => VisibilityKind::Private,
                _ => {
                    return Err(LumaError::spanned(
                        ParserErrorKind::InvalidVisibility {
                            ident: vis_token.lexeme.clone(),
                        },
                        vis_token.span,
                    ));
                }
            };

            self.advance(); // consume visibility token

            self.consume(TokenKind::RightParen)?;

            return Ok(Visibility::spanned(vis_token.span, visibility_kind));
        }

        // it's just 'pub'
        Ok(Visibility::spanned(pub_token.span, VisibilityKind::Public))
    }
}
