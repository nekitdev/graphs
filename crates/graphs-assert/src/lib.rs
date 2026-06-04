use proc_macro::TokenStream;
use syn::{Error, parse_macro_input};

mod check;
mod context;
mod generate;
mod name;
mod parse;

/// Asserts that the graph satisfies the given trait bounds.
#[proc_macro]
pub fn assert_graph(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Explicit);

    generate::assert_graph(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`Directed`] bound.
///
/// [`Directed`]: graphs_core::base::Directed
#[proc_macro]
pub fn assert_directed(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_directed(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`Undirected`] bound.
///
/// [`Undirected`]: graphs_core::base::Undirected
#[proc_macro]
pub fn assert_undirected(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_undirected(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`ForbidLoop`] bound.
///
/// [`ForbidLoop`]: graphs_core::base::ForbidLoop
#[proc_macro]
pub fn assert_forbid_loop(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_forbid_loop(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`AllowLoop`] bound.
///
/// [`AllowLoop`]: graphs_core::base::AllowLoop
#[proc_macro]
pub fn assert_allow_loop(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_allow_loop(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`SingleType`] bound.
///
/// [`SingleType`]: graphs_core::base::SingleType
#[proc_macro]
pub fn assert_single_type(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_single_type(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`MultipleType`] bound.
///
/// [`MultipleType`]: graphs_core::base::MultipleType
#[proc_macro]
pub fn assert_multiple_type(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_multiple_type(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`Simple`] bound.
///
/// [`Simple`]: graphs_core::base::Simple
#[proc_macro]
pub fn assert_simple(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_simple(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`Looped`] bound.
///
/// [`Looped`]: graphs_core::base::Looped
#[proc_macro]
pub fn assert_looped(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_looped(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`Multi`] bound.
///
/// [`Multi`]: graphs_core::base::Multi
#[proc_macro]
pub fn assert_multi(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_multi(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

/// Asserts that the graph satisfies the [`Pseudo`] bound.
///
/// [`Pseudo`]: graphs_core::base::Pseudo
#[proc_macro]
pub fn assert_pseudo(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as parse::Implicit);

    generate::assert_pseudo(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
