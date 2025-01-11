use either_of::Either;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use regex_syntax::{
    self,
    hir::{self, Hir, HirKind},
};
use syn::Ident;

pub struct ParseOptions {
    pub case_insensitive: bool,
    pub multi_line: bool,
    pub dot_matches_new_line: bool,
    pub ignore_whitespace: bool,
    pub unicode: bool,
    pub crlf: bool,
}

impl Default for ParseOptions {
    fn default() -> Self {
        Self {
            case_insensitive: false,
            multi_line: false,
            dot_matches_new_line: false,
            ignore_whitespace: false,
            unicode: true,
            crlf: false,
        }
    }
}

pub fn regex_to_code(
    name: &str,
    pattern: &str,
    options: ParseOptions,
) -> Result<String, Box<regex_syntax::Error>> {
    let name = Ident::new(name, proc_macro2::Span::call_site());
    regex_to_tokens(name, pattern, options).map(|ts| ts.to_string())
}

pub fn regex_to_tokens(
    name: Ident,
    pattern: &str,
    options: ParseOptions,
) -> Result<TokenStream, Box<regex_syntax::Error>> {
    let hir = regex_syntax::ParserBuilder::default()
        .case_insensitive(options.case_insensitive)
        .multi_line(options.multi_line)
        .dot_matches_new_line(options.dot_matches_new_line)
        .ignore_whitespace(options.ignore_whitespace)
        .unicode(options.unicode)
        .crlf(options.crlf)
        .build()
        .parse(pattern)?;

    let hir_dbg_repr = format!("{:#?}", hir.kind());
    let hir_dbg_repr = hir_dbg_repr.lines().map(|line| {
        quote! { #[doc = #line] }
    });

    let props_dbg_repr = format!("{:#?}", hir.properties());
    let props_dbg_repr = props_dbg_repr.lines().map(|line| {
        quote! { #[doc = #line] }
    });

    let parser: TokenStream = implement_parser(&hir);

    Ok(quote! {
        #[doc="Regex:"]
        #[doc="```text"]
        #[doc=#pattern]
        #[doc="```"]
        #[doc="HIR:"]
        #[doc="```no_run"]
        #(#hir_dbg_repr)*
        #[doc="```"]
        #[doc="HIR properties:"]
        #[doc="```no_run"]
        #(#props_dbg_repr)*
        #[doc="```"]
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
        HirKind::Class(class) => implement_class_parser(class),
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
            input
                .as_bytes()
                .windows(LITERAL.len())
                .any(|window| window == LITERAL)
        }
    }
}

fn implement_class_parser(class: &hir::Class) -> TokenStream {
    fn range_to_test<R>(range: &R) -> TokenStream
    where
        R: Range,
        R::Bound: ToTokens,
    {
        let start = range.start();
        let end = range.end();
        quote! { #start <= c && c <= #end }
    }

    let (iter, test_in_range) = match class {
        hir::Class::Bytes(class_bytes) => (
            quote! { str::as_bytes(input) },
            Either::Left(class_bytes.iter().map(range_to_test)),
        ),
        hir::Class::Unicode(class_unicode) => (
            quote! { str::chars(input) },
            Either::Right(class_unicode.iter().map(range_to_test)),
        ),
    };

    quote! {
        pub fn is_match(input: &str) -> bool {
            #iter.any(|c| false #(|| (#test_in_range))*)
        }
    }
}

trait Range {
    type Bound;

    fn start(&self) -> Self::Bound;
    fn end(&self) -> Self::Bound;
}

impl Range for hir::ClassBytesRange {
    type Bound = u8;

    fn start(&self) -> u8 {
        self.start()
    }

    fn end(&self) -> u8 {
        self.end()
    }
}

impl Range for hir::ClassUnicodeRange {
    type Bound = char;

    fn start(&self) -> char {
        self.start()
    }

    fn end(&self) -> char {
        self.end()
    }
}

fn implement_capture_parser(_index: u32, _name: Option<&str>, sub: &Hir) -> TokenStream {
    // TODO: use the index and name
    implement_parser(sub)
}
