use crate::{
    connection::Connection,
    control::Flow,
    id::{DefaultNodeId, NodeTypeId},
    neighbors::Neighbors,
    time::{Time, Timed},
    visit::{Visit, Visitor},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event<N: NodeTypeId = DefaultNodeId> {
    Discover(Timed<N>),
    Tree(Connection<N>),
    Back(Connection<N>),
    CrossOrForward(Connection<N>),
    Finish(Timed<N>),
}

pub use Event::{Back, CrossOrForward, Discover, Finish, Tree};

pub fn dfs<G, S, V, F>(graph: G, starting: S, mut visitor: V) -> F
where
    G: Visit + Neighbors,
    S: IntoIterator<Item = G::NodeId>,
    V: FnMut(Event<G::NodeId>) -> F,
    F: Flow<G::NodeId>,
{
    let mut time = Time::start();

    let mut discovered = graph.build_visitor();
    let mut finished = graph.build_visitor();

    for start in starting {
        control_flow!(dfs_recursive(
            &graph,
            start,
            &mut visitor,
            &mut discovered,
            &mut finished,
            &mut time,
        ), {
            // recursive function does not prune, so this should never happen
            prune => unreachable!()
        });
    }

    F::continuing()
}

macro_rules! prune_on_finish {
    () => {
        panic!("pruning on `finish` is not supported")
    };
}

pub(crate) fn dfs_recursive<G, V, F>(
    graph: &G,
    node: G::NodeId,
    visitor: &mut V,
    discovered: &mut G::Visitor,
    finished: &mut G::Visitor,
    time: &mut Time,
) -> F
where
    G: Visit + Neighbors + ?Sized,
    V: FnMut(Event<G::NodeId>) -> F,
    F: Flow<G::NodeId>,
{
    if !discovered.visit(node) {
        // already visited, continue
        return F::continuing();
    }

    // `discover` event

    let discover = Discover(Timed::new(node, time.increment()));

    control_flow!(visitor(discover), {
        continue => {
            for neighbor in graph.neighbors(node) {
                if !discovered.was_visited(neighbor) {
                    // `tree` event

                    let tree = Tree(Connection::new(node, neighbor));

                    control_flow!(visitor(tree));

                    control_flow!(dfs_recursive(
                        graph,
                        neighbor,
                        visitor,
                        discovered,
                        finished,
                        time,
                    ), {
                        // this function does not prune, so this should never happen

                        prune => unreachable!()
                    })
                } else if !finished.was_visited(neighbor) {
                    // `back` event

                    let back = Back(Connection::new(node, neighbor));

                    control_flow!(visitor(back));
                } else {
                    // unfortunately, we do not keep track of discovery times
                    // as that would require allocating, which is impossible
                    // without depending on `std` or `alloc`

                    // `cross` or `forward` event (depends on discovery time)

                    let cross_or_forward = CrossOrForward(Connection::new(node, neighbor));

                    control_flow!(visitor(cross_or_forward));
                }
            }
        },
        prune => {},
    });

    if finished.visit(node) {
        // `finish` event

        let finish = Finish(Timed::new(node, time.increment()));

        control_flow!(visitor(finish), {
            prune => prune_on_finish!(),
        });

        F::continuing()
    } else {
        // visitors are only accessible in this function, so this should never happen

        unreachable!()
    }
}
