use proc_macro::TokenStream;
use syn::{Error, LitStr, parse_macro_input};

mod expand;

#[proc_macro]
pub fn include_graph(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    let string = literal.value();

    expand::try_graph(string)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[proc_macro]
pub fn include_graph_str(input: TokenStream) -> TokenStream {
    let literal = parse_macro_input!(input as LitStr);

    let string = literal.value();

    expand::try_graph_str(string)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
