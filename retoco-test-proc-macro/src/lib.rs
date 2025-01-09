use arbitrary::{Arbitrary, Unstructured};
use proc_macro::TokenStream;
use prop::{strategy::ValueTree, test_runner::TestRunner};
use proptest::prelude::*;
use quote::quote;
use regex_syntax::{
    ast::Ast,
    hir::translate::Translator,
};

#[proc_macro]
pub fn test_cases(input: TokenStream) -> TokenStream {
    if !input.is_empty() {
        return syn::Error::new_spanned(
            proc_macro2::TokenStream::from(input),
            "This macro takes no arguments",
        )
        .to_compile_error()
        .into();
    }

    let mut test_cases = Vec::with_capacity(10);

    let raw_data_strategy = proptest::collection::vec(any::<u8>(), 0..=4096);
    for i in 0..10 {
        loop {
            let raw_data = raw_data_strategy
                .new_tree(&mut TestRunner::default())
                .unwrap()
                .current();
            let mut unstructured = Unstructured::new(&raw_data);
            let Ok(ast) = Ast::arbitrary(&mut unstructured) else {
                continue;
            };
            let mut translator = Translator::new();
            let Ok(hir) = translator.translate(&ast.to_string(), &ast) else {
                continue;
            };
            let pattern = hir.to_string();

            let test_case_ident = quote::format_ident!("test_case_{i}");
            test_cases.push(quote! {
                #[test]
                fn #test_case_ident() {
                    ::retoco::regex!(case = #pattern);
                    let case = ::regex::Regex::new(#pattern).unwrap();


                }
            });
        }
    }

    (quote! {
        #(#test_cases)*
    }).into()
}
