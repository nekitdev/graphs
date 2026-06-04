use std::collections::HashSet;

use crate::ast::{
    self, Attribute, AttributeStatement, Attributes, Id, Item, NodeId, NodeStatement,
};

pub struct Graph<'a> {
    pub strict: bool,
    pub id: Option<Id<'a>>,
    pub statements: Statements<'a>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Statements<'a> {
    pub list: Vec<Statement<'a>>,
}

impl<'a> From<ast::Statements<'a>> for Statements<'a> {
    fn from(statements: ast::Statements<'a>) -> Self {
        let mut list = Vec::new();

        for statement in statements.list {
            match statement {
                ast::Statement::Node(node_statement) => {
                    list.push(Statement::Node(node_statement));
                }
                ast::Statement::Edge(edge_statement) => {
                    // list.push(Self::Edge(edge_statement));
                }
                ast::Statement::Attribute(attribute_statement) => {
                    list.push(Statement::Attribute(attribute_statement));
                }
                ast::Statement::PlainAttribute(attribute) => {
                    list.push(Statement::PlainAttribute(attribute));
                }
                ast::Statement::Subgraph(graph) => {
                    let mut subgraph_statements = graph.statements.into();

                    list.append(&mut subgraph_statements);
                }
            }
        }

        Self { list }
    }
}

impl<'a> From<ast::EdgeStatement<'a>> for Vec<EdgeStatement<'a>> {
    fn from(statement: ast::EdgeStatement<'a>) -> Self {
        let mut statements = Self::new();

        let flattened = statement.flatten();

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
pub enum Statement<'a> {
    Node(NodeStatement<'a>),
    Edge(EdgeStatement<'a>),
    Attribute(AttributeStatement<'a>),
    PlainAttribute(Attribute<'a>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EdgeStatement<'a> {
    pub item: NodeId<'a>,
    pub tail: Tail<'a>,
    pub attributes: Option<Attributes<'a>>,
}

impl<'a> EdgeStatement<'a> {
    pub fn get_node_identifiers(&self) -> HashSet<NodeId<'a>> {
        let mut identifiers = self.tail.get_node_identifiers();

        identifiers.insert(self.item.clone());

        identifiers
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tail<'a> {
    pub item: NodeId<'a>,
    pub next: Option<Box<Tail<'a>>>,
}

impl<'a> Tail<'a> {
    pub fn get_node_identifiers(&self) -> HashSet<NodeId<'a>> {
        let mut identifiers = self
            .next
            .as_ref()
            .map(|next| next.get_node_identifiers())
            .unwrap_or_default();

        identifiers.insert(self.item.clone());

        identifiers
    }
}
