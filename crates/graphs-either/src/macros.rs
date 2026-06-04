#[macro_export]
macro_rules! either {
    (
        $either: expr,
        $node_pattern: pat => $node_output: expr,
        $edge_pattern: pat => $edge_output: expr $(,)?
    ) => {
        match $either {
            $crate::either::Node($node_pattern) => $node_output,
            $crate::either::Edge($edge_pattern) => $edge_output,
        }
    };
}

#[macro_export]
macro_rules! map_either {
    (
        $item: expr,
        $node_pattern: pat => $node_output: expr,
        $edge_pattern: pat => $edge_output: expr $(,)?
    ) => {
        $crate::either!(
            $item,
            $node_pattern => $crate::either::Node($node_output),
            $edge_pattern => $crate::either::Edge($edge_output),
        )
    };
}

#[macro_export]
macro_rules! both {
    ($item: expr, $both_pattern: pat => $both_output: expr) => {
        $crate::either!(
            $item,
            $both_pattern => $both_output,
            $both_pattern => $both_output,
        )
    };
}

#[macro_export]
macro_rules! map_both {
    ($item: expr, $both_pattern: pat => $both_output: expr) => {
        $crate::map_either!(
            $item,
            $both_pattern => $both_output,
            $both_pattern => $both_output,
        )
    }
}

#[macro_export]
macro_rules! factor_iter {
    ($either: expr) => {
        $crate::factoring::FactorIterator::new(
            $crate::map_both!($either, inner => inner.into_iter())
        )
    }
}

macro_rules! node_and_then {
    ($item: expr, $node_pattern: pat => $node_output: expr) => {
        $crate::either!(
            $item,
            $node_pattern => $node_output,
            value => $crate::either::Edge(value),
        )
    }
}

pub(crate) use node_and_then;

macro_rules! edge_and_then {
    ($item: expr, $edge_pattern: pat => $edge_output: expr) => {
        $crate::either!(
            $item,
            value => $crate::either::Node(value),
            $edge_pattern => $edge_output,
        )
    }
}

pub(crate) use edge_and_then;
