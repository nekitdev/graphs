//! Recursive depth-first search (DFS).
//!
//! The core of this module is the [`dfs`] function, which recursively traverses the given `graph`
//! from provided `starting` nodes, reporting DFS *events* to the supplied `function`.

use graphs_core::neighbors::Neighbors;

use crate::{
    control::Flow,
    dfs::{
        events::{Event, EventIn},
        internals::{finish_recursive, prune_on_finish, prune_recursive},
    },
    record::{
        Output::{Discovered, Finished, Undiscovered},
        Record, Recorder,
    },
    time::Time,
};

pub fn dfs<G, S, V, F>(graph: G, starting: S, mut visitor: V) -> F
where
    G: Record + Neighbors,
    S: IntoIterator<Item = G::NodeId>,
    V: FnMut(EventIn<G>) -> F,
    F: Flow,
{
    let mut time = Time::start();

    let mut recorder = graph.build_recorder();

    for start in starting {
        control_flow!(dfs_recursive(
            &graph,
            start,
            &mut visitor,
            &mut recorder,
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
    recorder: &mut G::Recorder,
    time: &mut Time,
) -> F
where
    G: Record + Neighbors + ?Sized,
    V: FnMut(EventIn<G>) -> F,
    F: Flow,
{
    if recorder.query(node).is_discovered() {
        // already visited, continue
        return F::continuing();
    }

    let discovered = time.strict_post_increment();

    recorder.discover(node, discovered);

    // `discover` event
    let discover = Event::discover(node, discovered);

    control_flow!(visitor(discover), {
        continue => {
            for neighbor in graph.neighbors(node) {
                match recorder.query(neighbor) {
                    Undiscovered => {
                        let tree = Event::tree(node, neighbor);

                        control_flow!(visitor(tree));

                        control_flow!(dfs_recursive(
                            graph, node, visitor, recorder, time
                        ), {
                            // this function does not prune, so this should never happen
                            prune => prune_recursive()
                        });
                    },
                    Discovered(_) => {
                        let back = Event::back(node, neighbor);

                        control_flow!(visitor(back));
                    }
                    Finished(times) => {
                        if discovered < times.discovered {
                            let forward = Event::forward(node, neighbor);

                            control_flow!(visitor(forward));
                        } else {
                            let cross = Event::cross(node, neighbor);

                            control_flow!(visitor(cross));
                        }
                    }
                }
            }
        },
        prune => {},
    });

    if recorder.query(node).is_finished() {
        // visitors are only accessible in this function, so this should never happen

        // reaching this most likely means that the `Visitor` implementation is incorrect
        finish_recursive()
    }

    let finished = time.strict_post_increment();

    recorder.finish(node, finished);

    let finish = Event::finish(node, finished);

    control_flow!(visitor(finish), {
        // pruning on `finish` is not supported, therefore panicking is needed
        prune => prune_on_finish(),
    });

    F::continuing()
}
