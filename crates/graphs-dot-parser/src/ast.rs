use std::borrow::Cow;

use non_empty_slice::{NonEmptyVec, non_empty_vec};
use ownership::IntoOwned;

use crate::parser::{DotError, DotPair, Parse, ParseError, Rule, parse_str};

pub(crate) mod import {
    pub use std::result::Result;
}

pub type Str<'a> = Cow<'a, str>;

fn as_str<'a>(pair: DotPair<'a>) -> Str<'a> {
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

impl<'a> Parse<'a> for Id<'a> {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub enum DotGraph<'a> {
    Directed(Graph<'a>),
    Undirected(Graph<'a>),
}

impl<'a> DotGraph<'a> {
    pub fn parse_str(input: &'a str) -> Result<Self, DotError> {
        parse_str(Rule::dotgraph, input)
    }
}

impl<'a> Parse<'a> for DotGraph<'a> {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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

impl<'a> Graphs<'a> {
    pub fn parse_str(input: &'a str) -> Result<Self, DotError> {
        parse_str(Rule::dotfile, input)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub struct Attribute<'a> {
    pub key: Id<'a>,
    pub value: Id<'a>,
}

impl<'a> Parse<'a> for Attribute<'a> {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    pub lists: Vec<AttributeList<'a>>,
}

impl<'a> Parse<'a> for Attributes<'a> {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
        let mut inner = input.clone().into_inner();

        let head = next_expected!(inner, input, attribute_list)?;

        let mut list = AttributeList::parse(head)?;

        let mut lists = vec![list];

        for pair in inner {
            list = AttributeList::parse(pair)?;

            lists.push(list);
        }

        let attributes = Self { lists };

        Ok(attributes)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, IntoOwned)]
pub enum AttributeStatement<'a> {
    Graph(Attributes<'a>),
    Node(Attributes<'a>),
    Edge(Attributes<'a>),
}

impl<'a> Parse<'a> for AttributeStatement<'a> {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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

impl<'a> Parse<'a> for NodeStatement<'a> {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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

impl<'a> Parse<'a> for NodeId<'a> {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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

impl<'a> Parse<'a> for Next<'a> {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>> {
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
