use graphs_core::{
    neighbors::Neighbors,
    visit::{Visit, Visitor},
};

use crate::{
    control::Flow,
    dfs::{
        events::{SimpleEvent, SimpleEventIn},
        internals::{finish_recursive, prune_on_finish, prune_recursive},
    },
    time::Time,
};

#[track_caller]
pub fn dfs<G, S, V, F>(graph: G, starting: S, mut visitor: V) -> F
where
    G: Visit + Neighbors,
    S: IntoIterator<Item = G::NodeId>,
    V: FnMut(SimpleEventIn<G>) -> F,
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
            prune => prune_recursive()
        });
    }

    F::continuing()
}

#[track_caller]
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
    V: FnMut(SimpleEventIn<G>) -> F,
    F: Flow,
{
    if discovered.visit(node).is_previously() {
        // already visited, continue
        return F::continuing();
    }

    // `discover` event
    let discover = SimpleEvent::discover(node, time.strict_post_increment());

    control_flow!(visitor(discover), {
        continue => {
            for neighbor in graph.neighbors(node) {
                if !discovered.was_visited(neighbor) {
                    // `tree` event
                    let tree = SimpleEvent::tree(node, neighbor);

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
                        prune => prune_recursive()
                    });
                } else if !finished.was_visited(neighbor) {
                    // `back` event
                    let back = SimpleEvent::back(node, neighbor);

                    control_flow!(visitor(back));
                } else {
                    // `cross` or `forward` event (depends on discovery time)
                    let cross_or_forward = SimpleEvent::cross_or_forward(node, neighbor);

                    control_flow!(visitor(cross_or_forward));
                }
            }
        },
        prune => {},
    });

    if finished.visit(node).is_previously() {
        // visitors are only accessible in this function, so this should never happen

        // reaching this most likely means that the `Visitor` implementation is incorrect
        finish_recursive()
    }

    // `finish` event
    let finish = SimpleEvent::finish(node, time.strict_post_increment());

    control_flow!(visitor(finish), {
        // pruning on `finish` is not supported, therefore panicking is needed
        prune => prune_on_finish(),
    });

    F::continuing()
}
