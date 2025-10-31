use crate::{
    base::Base,
    connections::Connection,
    control::Flow,
    neighbors::Neighbors,
    time::{Time, Timed},
    visit::{Visit, Visitor},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Event<C: Connection> {
    Discover(Timed<C::Id>),
    Tree(C),
    Back(C),
    CrossOrForward(C),
    Finish(Timed<C::Id>),
}

impl<C: Connection> Event<C> {
    pub fn discover(id: C::Id, time: Time) -> Self {
        Self::Discover(Timed::new(id, time))
    }

    pub fn tree(node: C::Id, neighbor: C::Id) -> Self {
        Self::Tree(C::connecting(node, neighbor))
    }

    pub fn back(node: C::Id, neighbor: C::Id) -> Self {
        Self::Back(C::connecting(node, neighbor))
    }

    pub fn cross_or_forward(node: C::Id, neighbor: C::Id) -> Self {
        Self::CrossOrForward(C::connecting(node, neighbor))
    }

    pub fn finish(id: C::Id, time: Time) -> Self {
        Self::Finish(Timed::new(id, time))
    }
}

pub use Event::{Back, CrossOrForward, Discover, Finish, Tree};

pub type EventIn<G> = Event<<G as Base>::Connection>;

pub fn dfs<G, S, V, F>(graph: G, starting: S, mut visitor: V) -> F
where
    G: Visit + Neighbors,
    S: IntoIterator<Item = G::NodeId>,
    V: FnMut(EventIn<G>) -> F,
    F: Flow,
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
    V: FnMut(EventIn<G>) -> F,
    F: Flow,
{
    if !discovered.visit(node) {
        // already visited, continue
        return F::continuing();
    }

    // `discover` event

    let discover = Event::discover(node, time.increment());

    control_flow!(visitor(discover), {
        continue => {
            for neighbor in graph.neighbors(node) {
                if !discovered.was_visited(neighbor) {
                    // `tree` event

                    let tree = Event::tree(node, neighbor);

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

                    let back = Event::back(node, neighbor);

                    control_flow!(visitor(back));
                } else {
                    // unfortunately, we do not keep track of discovery times
                    // as that would require allocating, which is impossible
                    // without depending on `std` or `alloc`

                    // `cross` or `forward` event (depends on discovery time)

                    let cross_or_forward = Event::cross_or_forward(node, neighbor);

                    control_flow!(visitor(cross_or_forward));
                }
            }
        },
        prune => {},
    });

    if finished.visit(node) {
        // `finish` event

        let finish = Event::finish(node, time.increment());

        control_flow!(visitor(finish), {
            prune => prune_on_finish!(),
        });

        F::continuing()
    } else {
        // visitors are only accessible in this function, so this should never happen

        unreachable!()
    }
}
