use proc_macro2::TokenStream;
use quote::quote;
use regex_syntax::hir::{self, Hir, HirKind};
use syn::Ident;

pub fn regex_to_code(name: &str, pattern: &str) -> Result<String, Box<regex_syntax::Error>> {
    let name = Ident::new(name, proc_macro2::Span::call_site());
    regex_to_tokens(name, pattern).map(|ts| ts.to_string())
}

pub fn regex_to_tokens(
    name: Ident,
    pattern: &str,
) -> Result<TokenStream, Box<regex_syntax::Error>> {
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
        mod #name {
            #parser
        }
    })
}

fn implement_parser(hir: &Hir) -> TokenStream {
    if hir.properties().minimum_len().is_none() {
        return quote! {
            pub const fn is_match(input: &str) -> bool {
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

fn implement_empty_parser() -> TokenStream {
    quote! {
        pub const fn is_match(_: &str) -> bool {
            true
        }
    }
}

fn implement_literal_parser(literal: &[u8]) -> TokenStream {
    assert!(!literal.is_empty());
    let lit_len = literal.len();

    quote! {
        pub fn is_match(input: &str) -> bool {
            const LITERAL: [u8; #lit_len] = [#(#literal),*];
            let input = input.as_bytes();

            if input.len() < LITERAL.len() {
                return false;
            }

            for i in 0..input.len() - LITERAL.len() + 1 {
                if input[i..][0..LITERAL.len()] == LITERAL {
                    return true;
                }
            }

            false
        }
    }
}

fn implement_capture_parser(_index: u32, _name: Option<&str>, sub: &Hir) -> TokenStream {
    // TODO: use the index and name
    implement_parser(sub)
}
