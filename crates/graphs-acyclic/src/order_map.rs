cfg_select! {
    feature = "std" => {
        use std::collections::BTreeMap;
    }
    _ => {
        use alloc::collections::BTreeMap;
    }
}

use graphs_core::id::NodeTypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct TopologicalPosition {
    position: usize,
}

pub const ZERO: usize = 0;

impl TopologicalPosition {
    pub const fn new(position: usize) -> Self {
        Self { position }
    }

    pub const fn get(self) -> usize {
        self.position
    }

    pub const ZERO: Self = Self::new(ZERO);
}

pub struct OrderMap<N: NodeTypeId> {
    position_to_node: BTreeMap<TopologicalPosition, N>,
    node_to_position: Vec<TopologicalPosition>,
}
