use proc_macro2::TokenStream;
use quote::quote;
use regex_syntax::hir::{self, Hir, HirKind};
use syn::Ident;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0}")]
    RegexSyntax(
        #[from]
        #[source]
        regex_syntax::Error,
    ),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn regex_to_tokens(pattern: &str, mod_name: Ident) -> Result<TokenStream> {
    let hir = regex_syntax::parse(pattern)?;

    let hir_dbg_repr = format!("{:#?}", hir.kind());
    let props_dbg_repr = format!("{:#?}", hir.properties());

    let parser: TokenStream = implement_parser(&hir);

    Ok(quote! {
        /// Regex:
        /// ```text
        #[doc=#pattern]
        /// ```
        ///
        /// HIR:
        /// ```no_run
        #[doc=#hir_dbg_repr]
        /// ```
        ///
        /// HIR properties:
        /// ```no_run
        #[doc=#props_dbg_repr]
        /// ```
        mod #mod_name {
            #parser
        }
    })
}

fn implement_parser(hir: &Hir) -> proc_macro2::TokenStream {
    if hir.properties().minimum_len().is_none() {
        return quote! {
            pub fn is_match(input: &str) -> bool {
                false
            }
        };
    }

    match hir.kind() {
        HirKind::Empty => implement_empty_parser(),
        HirKind::Literal(hir::Literal(lit)) => implement_literal_parser(lit),
        HirKind::Class(_) => todo!(),
        HirKind::Look(_) => todo!(),
        HirKind::Repetition(_) => todo!(),
        HirKind::Capture(hir::Capture { index, name, sub }) => {
            implement_capture_parser(*index, name.as_deref(), sub)
        }
        HirKind::Concat(_) => todo!(),
        HirKind::Alternation(_) => todo!(),
    }
}

fn implement_empty_parser() -> proc_macro2::TokenStream {
    quote! {
        pub fn is_match(_: &str) -> bool {
            true
        }
    }
}

fn implement_literal_parser(literal: &[u8]) -> proc_macro2::TokenStream {
    let body = if let Ok(literal) = std::str::from_utf8(literal) {
        quote! {
            input == #literal
        }
    } else {
        quote! {
            input.as_bytes() == &[#(#literal),*]
        }
    };

    quote! {
        pub fn is_match(input: &str) -> bool {
            #body
        }
    }
}

fn implement_capture_parser(_index: u32, _name: Option<&str>, sub: &Hir) -> proc_macro2::TokenStream {
    implement_parser(sub)
}
