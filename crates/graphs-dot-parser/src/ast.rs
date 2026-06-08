cfg_select! {
    feature = "std" => {
        use std::borrow::Cow;
    }
    _ => {
        use alloc::{borrow::Cow, boxed::Box, vec::Vec};
    }
}

use non_empty_slice::{NonEmptyVec, non_empty_vec};
use ownership::IntoOwned;

use crate::parser::{InternalError, Parse, ParsePair, Rule, Ruled};

pub(crate) mod import {
    pub use core::result::Result;
}

pub type Str<'a> = Cow<'a, str>;

fn as_str<'a>(pair: ParsePair<'a>) -> Str<'a> {
    Str::Borrowed(pair.as_str())
}

macro_rules! next_expected {
    ($inner: ident, $input: ident, $($expected: ident),+ $(,)?) => {
        $inner
            .next()
            .ok_or_else(|| $crate::parser::MissingPair::new($input.clone(), vec![
                $(Rule::$expected),+
            ]))
    };
}

macro_rules! rule_expected {
    ($found: ident, $($expected: ident),+ $(,)?) => {
        $crate::ast::import::Result::Err($crate::parser::ExpectedRule::new(vec![
            $(Rule::$expected),+
        ], $found))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub enum Id<'a> {
    Identifier(Str<'a>),
    Number(Str<'a>),
    Quoted(Str<'a>),
    Html(Str<'a>),
}

impl<'a> Id<'a> {
    pub fn get(self) -> Str<'a> {
        match self {
            Self::Identifier(string)
            | Self::Number(string)
            | Self::Quoted(string)
            | Self::Html(string) => string,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Identifier(string)
            | Self::Number(string)
            | Self::Quoted(string)
            | Self::Html(string) => string.as_ref(),
        }
    }
}

impl<'a> Parse<'a> for Id<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let pair = next_expected!(inner, input, identifier, number, quoted, html)?;

        let id = match pair.as_rule() {
            Rule::identifier => Self::Identifier(as_str(pair)),
            Rule::number => Self::Number(as_str(pair)),
            Rule::quoted => {
                // exclude quotes from the resulting id
                let mut inner = pair.into_inner();

                next_expected!(inner, input, quote)?;

                let string = next_expected!(inner, input, string)?;

                Self::Quoted(as_str(string))
            }
            Rule::html => Self::Html(as_str(pair)),
            found => rule_expected!(found, identifier, number, quoted, html)?,
        };

        Ok(id)
    }
}

pub use Id::{Html, Identifier, Number, Quoted};

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct Graph<'a> {
    pub strict: bool,
    pub id: Option<Id<'a>>,
    pub statements: Statements<'a>,
}

impl<'a> Graph<'a> {
    pub const fn statements(&self) -> &Statements<'a> {
        &self.statements
    }

    pub const fn statements_mut(&mut self) -> &mut Statements<'a> {
        &mut self.statements
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub enum DotGraph<'a> {
    Directed(Graph<'a>),
    Undirected(Graph<'a>),
}

impl<'a> DotGraph<'a> {
    pub const fn graph(&self) -> &Graph<'a> {
        match self {
            Self::Directed(graph) | Self::Undirected(graph) => graph,
        }
    }

    pub const fn graph_mut(&mut self) -> &mut Graph<'a> {
        match self {
            Self::Directed(graph) | Self::Undirected(graph) => graph,
        }
    }

    pub fn get(self) -> Graph<'a> {
        match self {
            Self::Directed(graph) | Self::Undirected(graph) => graph,
        }
    }
}

impl Ruled for DotGraph<'_> {
    const RULE: Rule = Rule::dotgraph;
}

impl<'a> Parse<'a> for DotGraph<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let mut strict = false;
        let mut id = None;

        let mut pair = next_expected!(inner, input, strict, graph, digraph)?;

        if let Rule::strict = pair.as_rule() {
            strict = true;

            pair = next_expected!(inner, input, graph, digraph)?;
        }

        let digraph = match pair.as_rule() {
            Rule::digraph => true,
            Rule::graph => false,
            found => rule_expected!(found, graph, digraph)?,
        };

        pair = next_expected!(inner, input, id, statements)?;

        if let Rule::id = pair.as_rule() {
            let parsed = Id::parse(pair)?;

            id = Some(parsed);

            pair = next_expected!(inner, input, statements)?;
        }

        let statements = Statements::parse(pair)?;

        let graph = Graph {
            strict,
            id,
            statements,
        };

        let typed = if digraph {
            Self::Directed(graph)
        } else {
            Self::Undirected(graph)
        };

        Ok(typed)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct Graphs<'a> {
    pub non_empty: NonEmptyVec<DotGraph<'a>>,
}

impl<'a> Parse<'a> for Graphs<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let head = next_expected!(inner, input, dotfile)?;

        let mut graph = DotGraph::parse(head)?;

        let mut non_empty = non_empty_vec![graph];

        for pair in inner {
            graph = DotGraph::parse(pair)?;

            non_empty.push(graph);
        }

        let graphs = Self { non_empty };

        Ok(graphs)
    }
}

impl Ruled for Graphs<'_> {
    const RULE: Rule = Rule::dotfile;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct Attribute<'a> {
    pub key: Id<'a>,
    pub value: Id<'a>,
}

impl<'a> Attribute<'a> {
    pub const fn key(&self) -> &Id<'a> {
        &self.key
    }

    pub const fn value(&self) -> &Id<'a> {
        &self.value
    }

    pub const fn key_mut(&mut self) -> &mut Id<'a> {
        &mut self.key
    }

    pub const fn value_mut(&mut self) -> &mut Id<'a> {
        &mut self.value
    }

    pub fn into_pair(self) -> (Id<'a>, Id<'a>) {
        (self.key, self.value)
    }
}

impl<'a> Parse<'a> for Attribute<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let key_pair = next_expected!(inner, input, id)?;

        let key = Id::parse(key_pair)?;

        let value_pair = next_expected!(inner, input, id)?;

        let value = Id::parse(value_pair)?;

        let attribute = Self { key, value };

        Ok(attribute)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct AttributeList<'a> {
    pub non_empty: NonEmptyVec<Attribute<'a>>,
}

impl<'a> Parse<'a> for AttributeList<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let head = next_expected!(inner, input, attribute)?;

        let mut attribute = Attribute::parse(head)?;

        let mut non_empty = non_empty_vec![attribute];

        for pair in inner {
            attribute = Attribute::parse(pair)?;

            non_empty.push(attribute);
        }

        let attribute_list = Self { non_empty };

        Ok(attribute_list)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct Attributes<'a> {
    pub non_empty: NonEmptyVec<AttributeList<'a>>,
}

impl<'a> Parse<'a> for Attributes<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let pair = next_expected!(inner, input, attribute_list)?;

        let mut list = AttributeList::parse(pair)?;

        let mut non_empty = non_empty_vec![list];

        for pair in inner {
            list = AttributeList::parse(pair)?;

            non_empty.push(list);
        }

        let attributes = Self { non_empty };

        Ok(attributes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub enum AttributeStatement<'a> {
    Graph(Attributes<'a>),
    Node(Attributes<'a>),
    Edge(Attributes<'a>),
}

impl<'a> AttributeStatement<'a> {
    pub const fn attributes(&self) -> &Attributes<'a> {
        match self {
            Self::Graph(attributes) | Self::Node(attributes) | Self::Edge(attributes) => attributes,
        }
    }

    pub const fn attributes_mut(&mut self) -> &mut Attributes<'a> {
        match self {
            Self::Graph(attributes) | Self::Node(attributes) | Self::Edge(attributes) => attributes,
        }
    }

    pub fn get(self) -> Attributes<'a> {
        match self {
            Self::Graph(attributes) | Self::Node(attributes) | Self::Edge(attributes) => attributes,
        }
    }
}

impl<'a> Parse<'a> for AttributeStatement<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let kind_pair = next_expected!(inner, input, graph, node, edge)?;

        let kind = kind_pair.as_rule();

        let attributes_pair = next_expected!(inner, input, attributes)?;

        let attributes = Attributes::parse(attributes_pair)?;

        let attribute_statement = match kind {
            Rule::graph => Self::Graph(attributes),
            Rule::node => Self::Node(attributes),
            Rule::edge => Self::Edge(attributes),
            found => rule_expected!(found, graph, node, edge)?,
        };

        Ok(attribute_statement)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct NodeStatement<'a> {
    pub node: NodeId<'a>,
    pub attributes: Option<Attributes<'a>>,
}

impl<'a> NodeStatement<'a> {
    pub const fn node(&self) -> &NodeId<'a> {
        &self.node
    }

    pub const fn node_mut(&mut self) -> &mut NodeId<'a> {
        &mut self.node
    }
}

impl<'a> Parse<'a> for NodeStatement<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let node_pair = next_expected!(inner, input, node_id)?;

        let node = NodeId::parse(node_pair)?;

        let attributes = inner.next().map(Attributes::parse).transpose()?;

        let node_statement = Self { node, attributes };

        Ok(node_statement)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct EdgeStatement<'a> {
    pub from: Item<'a>,
    pub next: Next<'a>,
    pub attributes: Option<Attributes<'a>>,
}

impl<'a> EdgeStatement<'a> {
    pub const fn from(&self) -> &Item<'a> {
        &self.from
    }

    pub const fn from_mut(&mut self) -> &mut Item<'a> {
        &mut self.from
    }

    pub const fn next(&self) -> &Next<'a> {
        &self.next
    }

    pub const fn next_mut(&mut self) -> &mut Next<'a> {
        &mut self.next
    }

    pub fn flatten(self) -> Vec<EdgeStatement<'a>> {
        let mut from = self.from;
        let mut next = self.next;

        let attributes = self.attributes;

        let mut statements = Vec::new();

        loop {
            let statement = Self {
                from: from.clone(),
                next: Next {
                    to: next.to.clone(),
                    next: None,
                },
                attributes: attributes.clone(),
            };

            statements.push(statement);

            match next.next {
                Some(next_next) => {
                    from = next.to;
                    next = *next_next;
                }
                None => return statements,
            }
        }
    }
}

impl<'a> Parse<'a> for EdgeStatement<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let from_pair = next_expected!(inner, input, item)?;

        let from = Item::parse(from_pair)?;

        let next_pair = next_expected!(inner, input, next)?;

        let next = Next::parse(next_pair)?;

        let attributes = inner.next().map(Attributes::parse).transpose()?;

        let edge_statement = Self {
            from,
            next,
            attributes,
        };

        Ok(edge_statement)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub enum Item<'a> {
    Node(NodeId<'a>),
    Subgraph(Subgraph<'a>),
}

impl<'a> Parse<'a> for Item<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let pair = next_expected!(inner, input, node_id, subgraph)?;

        let item = match pair.as_rule() {
            Rule::node_id => {
                let node_id = NodeId::parse(pair)?;

                Self::Node(node_id)
            }
            Rule::subgraph => {
                let subgraph = Subgraph::parse(pair)?;

                Self::Subgraph(subgraph)
            }
            found => rule_expected!(found, node_id, subgraph)?,
        };

        Ok(item)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct NodeId<'a> {
    pub id: Id<'a>,
    pub port: Option<Port<'a>>,
}

impl<'a> NodeId<'a> {
    pub const fn id(&self) -> &Id<'a> {
        &self.id
    }

    pub const fn id_mut(&mut self) -> &mut Id<'a> {
        &mut self.id
    }
}

impl<'a> Parse<'a> for NodeId<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let id_pair = next_expected!(inner, input, id)?;

        let id = Id::parse(id_pair)?;

        let port = inner.next().map(Port::parse).transpose()?;

        let node_id = Self { id, port };

        Ok(node_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub enum Port<'a> {
    Id(Id<'a>, Option<Compass>),
    Compass(Compass),
}

impl<'a> Parse<'a> for Port<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let pair = next_expected!(inner, input, compass, id)?;

        let port = match pair.as_rule() {
            Rule::compass => {
                let compass = Compass::parse(pair)?;

                Self::Compass(compass)
            }
            Rule::id => {
                let id = Id::parse(pair)?;

                let option = inner.next().map(Compass::parse).transpose()?;

                Self::Id(id, option)
            }
            found => rule_expected!(found, compass, id)?,
        };

        Ok(port)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct Next<'a> {
    pub to: Item<'a>,
    pub next: Option<Box<Next<'a>>>,
}

impl<'a> Next<'a> {
    pub const fn to(&self) -> &Item<'a> {
        &self.to
    }

    pub const fn to_mut(&mut self) -> &mut Item<'a> {
        &mut self.to
    }
}

impl<'a> Parse<'a> for Next<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let pair = next_expected!(inner, input, item)?;

        let to = Item::parse(pair)?;

        let next = inner.next().map(Self::parse).transpose()?.map(Box::new);

        let tail = Self { to, next };

        Ok(tail)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct Subgraph<'a> {
    pub id: Option<Id<'a>>,
    pub statements: Statements<'a>,
}

impl<'a> Subgraph<'a> {
    pub const fn statements(&self) -> &Statements<'a> {
        &self.statements
    }

    pub const fn statements_mut(&mut self) -> &mut Statements<'a> {
        &mut self.statements
    }

    pub fn into_graph(self, strict: bool) -> Graph<'a> {
        Graph {
            strict,
            id: self.id,
            statements: self.statements,
        }
    }

    pub fn into_directed(self, strict: bool) -> DotGraph<'a> {
        DotGraph::Directed(self.into_graph(strict))
    }

    pub fn into_undirected(self, strict: bool) -> DotGraph<'a> {
        DotGraph::Undirected(self.into_graph(strict))
    }
}

impl<'a> Parse<'a> for Subgraph<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let mut id = None;

        let mut pair = next_expected!(inner, input, id, statements)?;

        if let Rule::id = pair.as_rule() {
            let parsed = Id::parse(pair)?;

            id = Some(parsed);

            pair = next_expected!(inner, input, statements)?;
        }

        let statements = Statements::parse(pair)?;

        let subgraph = Self { id, statements };

        Ok(subgraph)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct Statements<'a> {
    pub list: Vec<Statement<'a>>,
}

impl<'a> Parse<'a> for Statements<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let list = input
            .into_inner()
            .map(Statement::parse)
            .collect::<Result<_, _>>()?;

        let statements = Self { list };

        Ok(statements)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub enum Statement<'a> {
    Node(NodeStatement<'a>),
    Edge(EdgeStatement<'a>),
    Attribute(AttributeStatement<'a>),
    PlainAttribute(Attribute<'a>),
    Subgraph(Subgraph<'a>),
}

impl<'a> Parse<'a> for Statement<'a> {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let pair = next_expected!(
            inner,
            input,
            node_statement,
            edge_statement,
            attribute_statement,
            attribute,
            subgraph
        )?;

        let statement = match pair.as_rule() {
            Rule::node_statement => {
                let node_statement = NodeStatement::parse(pair)?;

                Self::Node(node_statement)
            }
            Rule::edge_statement => {
                let edge_statement = EdgeStatement::parse(pair)?;

                Self::Edge(edge_statement)
            }
            Rule::attribute_statement => {
                let attribute_statement = AttributeStatement::parse(pair)?;

                Self::Attribute(attribute_statement)
            }
            Rule::attribute => {
                let attribute = Attribute::parse(pair)?;

                Self::PlainAttribute(attribute)
            }
            Rule::subgraph => {
                let subgraph = Subgraph::parse(pair)?;

                Self::Subgraph(subgraph)
            }
            found => rule_expected!(
                found,
                node_statement,
                edge_statement,
                attribute_statement,
                attribute,
                subgraph
            )?,
        };

        Ok(statement)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, IntoOwned)]
pub enum Compass {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
    C,
    #[default]
    Unspecified,
}

impl<'a> Parse<'a> for Compass {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let mut inner = input.clone().into_inner();

        let pair = next_expected!(inner, input, n, ne, e, se, s, sw, w, nw, c, unspecified)?;

        let compass = match pair.as_rule() {
            Rule::n => Self::N,
            Rule::ne => Self::NE,
            Rule::e => Self::E,
            Rule::se => Self::SE,
            Rule::s => Self::S,
            Rule::sw => Self::SW,
            Rule::w => Self::W,
            Rule::nw => Self::NW,
            Rule::c => Self::C,
            Rule::unspecified => Self::Unspecified,
            found => rule_expected!(found, n, ne, e, se, s, sw, w, nw, c, unspecified)?,
        };

        Ok(compass)
    }
}

#[cfg(feature = "proc-macro")]
mod tokens {
    use proc_macro2::TokenStream;
    use quote::{ToTokens, quote};

    use super::{
        Attribute, AttributeList, AttributeStatement, Attributes, Compass, DotGraph, EdgeStatement,
        Graph, Graphs, Id, Item, Next, NodeId, NodeStatement, Port, Statement, Statements,
        Subgraph,
    };

    fn quote_str(string: &str) -> TokenStream {
        quote! {
            graphs_dot_parser::ast::Str::Borrowed(#string)
        }
    }

    impl Id<'_> {
        fn variant(&self) -> TokenStream {
            match self {
                Self::Identifier(_) => quote! {
                    Identifier
                },
                Self::Number(_) => quote! {
                    Number
                },
                Self::Quoted(_) => quote! {
                    Quoted
                },
                Self::Html(_) => quote! {
                    Html
                },
            }
        }
    }

    impl ToTokens for Id<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Id
            };

            let string = quote_str(self.as_str());

            let variant = self.variant();

            let tokens = quote! {
                #path::#variant(#string)
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Graph<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Graph
            };

            let strict = self.strict;

            let id = match self.id.as_ref() {
                Some(id) => quote! {
                    Some(#id)
                },
                None => quote! {
                    None
                },
            };

            let statements = self.statements();

            let tokens = quote! {
                #path {
                    strict: #strict,
                    id: #id,
                    statements: #statements,
                }
            };

            stream.extend(tokens);
        }
    }

    impl DotGraph<'_> {
        fn variant(&self) -> TokenStream {
            match self {
                Self::Directed(_) => quote! {
                    Directed
                },
                Self::Undirected(_) => quote! {
                    Undirected
                },
            }
        }
    }

    impl ToTokens for DotGraph<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::DotGraph
            };

            let variant = self.variant();

            let graph = self.graph();

            let tokens = quote! {
                #path::#variant(#graph)
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Graphs<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Graphs
            };

            let non_empty_slice = self.non_empty.as_non_empty_slice();

            let non_empty = quote! {
                non_empty_slice::non_empty_vec![
                    #(#non_empty_slice),*
                ]
            };

            let tokens = quote! {
                #path {
                    non_empty: #non_empty,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Attribute<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Attribute
            };

            let key = self.key();
            let value = self.value();

            let tokens = quote! {
                #path {
                    key: #key,
                    value: #value,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for AttributeList<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::AttributeList
            };

            let non_empty_slice = self.non_empty.as_non_empty_slice();

            let non_empty = quote! {
                non_empty_slice::non_empty_vec![
                    #(#non_empty_slice),*
                ]
            };

            let tokens = quote! {
                #path {
                    non_empty: #non_empty,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Attributes<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Attributes
            };

            let non_empty_slice = self.non_empty.as_non_empty_slice();

            let non_empty = quote! {
                non_empty_slice::non_empty_vec![
                    #(#non_empty_slice),*
                ]
            };

            let tokens = quote! {
                #path {
                    non_empty: #non_empty,
                }
            };

            stream.extend(tokens);
        }
    }

    impl AttributeStatement<'_> {
        fn variant(&self) -> TokenStream {
            match self {
                Self::Graph(_) => quote! {
                    Graph
                },
                Self::Node(_) => quote! {
                    Node
                },
                Self::Edge(_) => quote! {
                    Edge
                },
            }
        }
    }

    impl ToTokens for AttributeStatement<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::AttributeStatement
            };

            let variant = self.variant();

            let attributes = self.attributes();

            let tokens = quote! {
                #path::#variant(#attributes)
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for NodeStatement<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::NodeStatement
            };

            let node = self.node();

            let attributes = match self.attributes.as_ref() {
                Some(attributes) => quote! {
                    Some(#attributes)
                },
                None => quote! {
                    None
                },
            };

            let tokens = quote! {
                #path {
                    node: #node,
                    attributes: #attributes,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for EdgeStatement<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::EdgeStatement
            };

            let from = self.from();
            let next = self.next();

            let attributes = match self.attributes.as_ref() {
                Some(attributes) => quote! {
                    Some(#attributes)
                },
                None => quote! {
                    None
                },
            };

            let tokens = quote! {
                #path {
                    from: #from,
                    next: #next,
                    attributes: #attributes,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Item<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Item
            };

            let init = match self {
                Self::Node(node) => quote! {
                    Node(#node)
                },
                Self::Subgraph(subgraph) => quote! {
                    Subgraph(#subgraph)
                },
            };

            let tokens = quote! {
                #path::#init
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for NodeId<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::NodeId
            };

            let id = self.id();

            let port = match self.port.as_ref() {
                Some(port) => quote! {
                    Some(#port)
                },
                None => quote! {
                    None
                },
            };

            let tokens = quote! {
                #path {
                    id: #id,
                    port: #port,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Port<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Port
            };

            let init = match self {
                Self::Id(id, option) => match option {
                    Some(compass) => quote! {
                        Id(#id, Some(#compass))
                    },
                    None => quote! {
                        Id(#id, None)
                    },
                },
                Self::Compass(compass) => quote! {
                    Compass(#compass)
                },
            };

            let tokens = quote! {
                #path::#init
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Next<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Next
            };

            let to = self.to();

            let next = match self.next.as_deref() {
                Some(next) => quote! {
                    Some(Box::new(#next))
                },
                None => quote! {
                    None
                },
            };

            let tokens = quote! {
                #path {
                    to: #to,
                    next: #next,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Subgraph<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Subgraph
            };

            let id = match self.id.as_ref() {
                Some(id) => quote! {
                    Some(#id)
                },
                None => quote! {
                    None
                },
            };

            let statements = self.statements();

            let tokens = quote! {
                #path {
                    id: #id,
                    statements: #statements,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Statements<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Statements
            };

            let slice = self.list.as_slice();

            let list = quote! {
                vec![
                    #(#slice),*
                ]
            };

            let tokens = quote! {
                #path {
                    list: #list,
                }
            };

            stream.extend(tokens);
        }
    }

    impl ToTokens for Statement<'_> {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Statement
            };

            let init = match self {
                Self::Node(node) => quote! {
                    Node(#node)
                },
                Self::Edge(edge) => quote! {
                    Edge(#edge)
                },
                Self::Attribute(attribute) => quote! {
                    Attribute(#attribute)
                },
                Self::PlainAttribute(plain) => quote! {
                    PlainAttribute(#plain)
                },
                Self::Subgraph(subgraph) => quote! {
                    Subgraph(#subgraph)
                },
            };

            let tokens = quote! {
                #path::#init
            };

            stream.extend(tokens);
        }
    }

    impl Compass {
        fn variant(&self) -> TokenStream {
            match self {
                Self::N => quote! {
                    N
                },
                Self::NE => quote! {
                    NE
                },
                Self::E => quote! {
                    E
                },
                Self::SE => quote! {
                    SE
                },
                Self::S => quote! {
                    S
                },
                Self::SW => quote! {
                    SW
                },
                Self::W => quote! {
                    W
                },
                Self::NW => quote! {
                    NW
                },
                Self::C => quote! {
                    C
                },
                Self::Unspecified => quote! {
                    Unspecified
                },
            }
        }
    }

    impl ToTokens for Compass {
        fn to_tokens(&self, stream: &mut TokenStream) {
            let path = quote! {
                graphs_dot_parser::ast::Compass
            };

            let variant = self.variant();

            let tokens = quote! {
                #path::#variant
            };

            stream.extend(tokens);
        }
    }
}
