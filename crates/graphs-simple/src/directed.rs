use graphs_core::{
    index::{DefaultUntypedIndex, UntypedIndex},
    kinds::Directed,
    loops::{Allow, DefaultLoop, Forbid, Loop},
    reverse::Reverse,
    types::{DefaultType, Multiple, Single, Type},
};

use crate::generic::GenericGraph;

pub type DiGraph<N, E, I = DefaultUntypedIndex, T = DefaultType, L = DefaultLoop> =
    GenericGraph<N, E, I, Directed, T, L>;

pub type SimpleDiGraph<N, E, I = DefaultUntypedIndex> = DiGraph<N, E, I, Single, Forbid>;
pub type LoopedDiGraph<N, E, I = DefaultUntypedIndex> = DiGraph<N, E, I, Single, Allow>;
pub type MultiDiGraph<N, E, I = DefaultUntypedIndex> = DiGraph<N, E, I, Multiple, Forbid>;
pub type PseudoDiGraph<N, E, I = DefaultUntypedIndex> = DiGraph<N, E, I, Multiple, Allow>;

// impl<N, E, I: UntypedIndex> Build for PseudoDiGraph<N, E, I> {
//     type NodeError = NodeError;
//     type EdgeError = PseudoError<I>;

//     fn add_node(
//         &mut self,
//         value: Self::NodeValue,
//     ) -> RecoverableResult<Self::NodeId, Self::NodeError, Self::NodeValue> {
//         let id = Self::NodeId::of(self.node_count());

//         if id.is_limit() {
//             return recoverable_result!(NodeError::new(), value);
//         }

//         let node = Node::new(value);

//         self.nodes.push(node);

//         Ok(id)
//     }

//     fn add_edge(
//         &mut self,
//         connection: Connection<Self::NodeId>,
//         value: Self::EdgeValue,
//     ) -> RecoverableResult<Self::EdgeId, Self::EdgeError, Self::EdgeValue> {
//         let id = Self::EdgeId::of(self.edge_count());

//         if id.is_limit() {
//             return recoverable_result!(LimitError::new(), value);
//         }

//         if let Err(missing) = self.find(connection) {
//             return recoverable_result!(missing, value);
//         }

//         Ok(id)
//     }
// }

impl<N, E, I: UntypedIndex, T: Type, L: Loop> Reverse for DiGraph<N, E, I, T, L> {
    fn reverse(&mut self) {
        self.nodes.iter_mut().for_each(|node| node.reverse());
        self.edges.iter_mut().for_each(|edge| edge.reverse());
    }
}

#[allow(dead_code)]
mod assert {
    use graphs_core::{
        base::{assert_directed, assert_looped, assert_multi, assert_pseudo, assert_simple},
        index::UntypedIndex,
        loops::Loop,
        types::Type,
    };

    use super::{DiGraph, LoopedDiGraph, MultiDiGraph, PseudoDiGraph, SimpleDiGraph};

    const fn assert_on_base<N, E, I: UntypedIndex, T: Type, L: Loop>() {
        assert_directed::<DiGraph<N, E, I, T, L>>();
    }

    const fn assert_on_simple<N, E, I: UntypedIndex>() {
        assert_simple::<SimpleDiGraph<N, E, I>>();
    }

    const fn assert_on_looped<N, E, I: UntypedIndex>() {
        assert_looped::<LoopedDiGraph<N, E, I>>();
    }

    const fn assert_on_multi<N, E, I: UntypedIndex>() {
        assert_multi::<MultiDiGraph<N, E, I>>();
    }

    const fn assert_on_pseudo<N, E, I: UntypedIndex>() {
        assert_pseudo::<PseudoDiGraph<N, E, I>>();
    }
}
