use proc_macro2::TokenStream;
use quote::quote;
use syn::{Result, parse_quote};

use crate::{
    check::Checker,
    context::Context,
    name::Name,
    parse::{Bounds, Explicit, Implicit},
};

pub fn assert_graph(input: Explicit) -> Result<TokenStream> {
    let context = Context::new();

    let output = assert_graph_with(&context, input);

    context.check()?;

    Ok(output)
}

pub fn assert_graph_with(context: &Context, explicit: Explicit) -> TokenStream {
    let graph = Name::GRAPH_TYPE.ident();

    let mut checker = Checker::new(&graph, context);

    checker.visit_explicit(&explicit);

    let Explicit {
        name,
        generics,
        bounds,
    } = explicit;

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote! {
        const _: () = {
            use graphs_core as _graphs_core;

            const fn assertion #impl_generics () #where_clause {
                const fn assert<#graph: #bounds>() {}

                assert::<#name #type_generics>();
            }
        };
    }
}

pub fn assert_directed(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(directed))
}

pub fn assert_undirected(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(undirected))
}

pub fn assert_forbid_loop(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(forbid_loop))
}

pub fn assert_allow_loop(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(allow_loop))
}

pub fn assert_single_type(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(single_type))
}

pub fn assert_multiple_type(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(multiple_type))
}

pub fn assert_simple(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(simple))
}

pub fn assert_looped(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(looped))
}

pub fn assert_multi(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(multi))
}

pub fn assert_pseudo(input: Implicit) -> Result<TokenStream> {
    assert_graph(input.explicit_with(pseudo))
}

pub fn directed() -> Bounds {
    parse_quote! {
        _graphs_core::base::Directed
    }
}

pub fn undirected() -> Bounds {
    parse_quote! {
        _graphs_core::base::Undirected
    }
}

pub fn forbid_loop() -> Bounds {
    parse_quote! {
        _graphs_core::base::ForbidLoop
    }
}

pub fn allow_loop() -> Bounds {
    parse_quote! {
        _graphs_core::base::AllowLoop
    }
}

pub fn single_type() -> Bounds {
    parse_quote! {
        _graphs_core::base::SingleType
    }
}

pub fn multiple_type() -> Bounds {
    parse_quote! {
        _graphs_core::base::MultipleType
    }
}

pub fn simple() -> Bounds {
    parse_quote! {
        _graphs_core::base::Simple
    }
}

pub fn looped() -> Bounds {
    parse_quote! {
        _graphs_core::base::Looped
    }
}

pub fn multi() -> Bounds {
    parse_quote! {
        _graphs_core::base::Multi
    }
}

pub fn pseudo() -> Bounds {
    parse_quote! {
        _graphs_core::base::Pseudo
    }
}
