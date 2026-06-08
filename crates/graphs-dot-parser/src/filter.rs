use std::collections::HashSet;

use crate::ast::{
    Attribute, AttributeStatement, Attributes, EdgeStatement, Graph, Id, NodeId, NodeStatement,
    Statement, Statements,
};

pub struct FilterGraph<'a> {
    pub strict: bool,
    pub id: Option<Id<'a>>,
    pub statements: FilterStatements<'a>,
}

impl<'a> Graph<'a> {
    pub fn into_filter(self) -> FilterGraph<'a> {
        let strict = self.strict;
        let id = self.id;
        let statements = self.statements.into_filter();

        FilterGraph {
            strict,
            id,
            statements,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterStatements<'a> {
    pub list: Vec<FilterStatement<'a>>,
}

impl<'a> Statements<'a> {
    pub fn into_filter(self) -> FilterStatements<'a> {
        let mut list = Vec::new();

        for statement in self.list {
            match statement {
                Statement::Node(node_statement) => {
                    list.push(FilterStatement::Node(node_statement));
                }
                Statement::Edge(edge_statement) => {
                    let mut edge_statements = edge_statement
                        .into_filter()
                        .into_iter()
                        .map(FilterStatement::Edge)
                        .collect();

                    list.append(&mut edge_statements);
                }
                Statement::Attribute(attribute_statement) => {
                    list.push(FilterStatement::Attribute(attribute_statement));
                }
                Statement::PlainAttribute(attribute) => {
                    list.push(FilterStatement::PlainAttribute(attribute));
                }
                Statement::Subgraph(graph) => {
                    let mut subgraph_statements = graph.statements.into_filter();

                    list.append(&mut subgraph_statements.list);
                }
            }
        }

        FilterStatements { list }
    }
}

impl<'a> EdgeStatement<'a> {
    pub fn into_filter(self) -> Vec<FilterEdgeStatement<'a>> {
        let mut statements = Vec::new();

        let flattened = self.flatten();

        for flat in flattened {
            let from = flat.from;
            let to = flat.next.to;
            let attributes = flat.attributes;
        }

        statements
    }
}

// impl<A> From<crate::ast::EdgeStmt<A>> for Vec<EdgeStmt<A>>
// where
//     A: Clone,
// {
//     fn from(edge: crate::ast::EdgeStmt<A>) -> Vec<EdgeStmt<A>> {
//         let edges = edge.flatten();
//         let mut v: Vec<EdgeStmt<A>> = Vec::new();

//         for edge in edges {
//             // `edge` has just one destination (`edge.next.next` is `None`), as it comes from a
//             // "flattened" set of edges.
//             let from = edge.from;
//             let to = edge.next.to;
//             let attr = edge.attr;

//             let (from_ids, mut extra_edges_from) = match from {
//                 Either::Left(node_from) => (HashSet::from_iter([node_from]), Vec::new()),
//                 Either::Right(subgraph) => {
//                     let g: Graph<A> = subgraph.into_graph(false, false).into();
//                     (g.get_node_ids(), g.get_edges_stmts())
//                 }
//             };

//             let (to_ids, mut extra_edges_to) = match to {
//                 Either::Left(node_from) => (HashSet::from_iter([node_from]), Vec::new()),
//                 Either::Right(subgraph) => {
//                     let g: Graph<A> = subgraph.into_graph(false, false).into();
//                     (g.get_node_ids(), g.get_edges_stmts())
//                 }
//             };

//             for from in from_ids {
//                 for to in &to_ids {
//                     v.push(EdgeStmt {
//                         from: from.clone(),
//                         next: EdgeRHS {
//                             to: to.clone(),
//                             next: None,
//                         },
//                         attr: attr.clone(),
//                     });
//                 }
//             }
//             v.append(&mut extra_edges_from);
//             v.append(&mut extra_edges_to);
//         }
//         v
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilterStatement<'a> {
    Node(NodeStatement<'a>),
    Edge(FilterEdgeStatement<'a>),
    Attribute(AttributeStatement<'a>),
    PlainAttribute(Attribute<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterEdgeStatement<'a> {
    pub from: NodeId<'a>,
    pub next: FilterNext<'a>,
    pub attributes: Option<Attributes<'a>>,
}

impl<'a> FilterEdgeStatement<'a> {
    pub fn get_node_identifiers(&self) -> HashSet<NodeId<'a>> {
        let mut identifiers = self.next.get_node_identifiers();

        identifiers.insert(self.from.clone());

        identifiers
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilterNext<'a> {
    pub to: NodeId<'a>,
    pub next: Option<Box<FilterNext<'a>>>,
}

impl<'a> FilterNext<'a> {
    pub fn get_node_identifiers(&self) -> HashSet<NodeId<'a>> {
        let mut identifiers = self
            .next
            .as_ref()
            .map(|next| next.get_node_identifiers())
            .unwrap_or_default();

        identifiers.insert(self.to.clone());

        identifiers
    }
}
