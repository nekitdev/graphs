use std::path::Path;

use graphs_dot_parser::{
    ast::DotGraph,
    owned::OwnedGraph,
    parser::{FromFile, ParseStr},
};
use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;

pub fn try_graph<P: AsRef<Path>>(path: P) -> Result<TokenStream> {
    let owned = OwnedGraph::from_file(path)?;

    let graph = owned.get();

    let tokens = quote! {
        #graph
    };

    Ok(tokens)
}

pub fn try_graph_str<S: AsRef<str>>(string: S) -> Result<TokenStream> {
    let graph = DotGraph::parse_str(string.as_ref())?;

    let tokens = quote! {
        #graph
    };

    Ok(tokens)
}
