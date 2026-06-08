use ownership::IntoOwned;

use crate::{
    ast::DotGraph,
    parser::{InternalError, Parse, ParsePair, Rule, Ruled},
};

pub type StaticGraph = DotGraph<'static>;

pub struct OwnedGraph {
    pub graph: StaticGraph,
}

impl OwnedGraph {
    pub const fn new(graph: StaticGraph) -> Self {
        Self { graph }
    }

    pub fn get(self) -> StaticGraph {
        self.graph
    }
}

impl<'a> Parse<'a> for OwnedGraph {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>> {
        let graph = DotGraph::parse(input)?;

        let owned = graph.into_owned();

        let file = Self::new(owned);

        Ok(file)
    }
}

impl Ruled for OwnedGraph {
    const RULE: Rule = StaticGraph::RULE;
}
