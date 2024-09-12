use proc_macro::TokenStream;
use retoco_token_stream::regex_to_tokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitStr,
};

#[proc_macro]
pub fn regex(input: TokenStream) -> TokenStream {
    let RegexDeclaration { name, pattern } = parse_macro_input!(input as RegexDeclaration);

    let tokens = match regex_to_tokens(&pattern.value(), name) {
        Ok(tokens) => tokens,
        Err(err) => match err {
            retoco_token_stream::Error::RegexSyntax(err) => {
                syn::Error::new_spanned(pattern, err).to_compile_error()
            }
        },
    };

    tokens.into()
}

struct RegexDeclaration {
    name: Ident,
    pattern: LitStr,
}

impl Parse for RegexDeclaration {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name = input.parse()?;
        let _: syn::token::Eq = input.parse()?;
        let pattern = input.parse()?;

        Ok(Self { name, pattern })
    }
}
