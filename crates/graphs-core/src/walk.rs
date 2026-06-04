//! Walkers are iterators which have context supplied to them.
//!
//! This allows, for one, traversing graphs without holding onto their borrow longer than needed.
//!
//! For instance, instead of defining depth-first search like this:
//!
//! ```
//! use graphs_core::{
//!     neighbors::Neighbors,
//!     visit::{Visit, Visitor},
//! };
//!
//! struct Dfs<'g, G: Visit> {
//!     graph: &'g G,
//!     stack: Vec<G::NodeId>,
//!     discovery: G::Visitor,
//! }
//!
//! impl<'g, G: Visit> Dfs<'g, G> {
//!     fn new(graph: &'g G, start: N) -> Self {
//!         let stack = vec![start];
//!
//!         let discovery = graph.build_visitor();
//!
//!         Self { graph, stack, discovery }
//!     }
//! }
//!
//! impl<'g, G: Visit + Neighbors> Iterator for Dfs<'g, G> {
//!     type Item = G::NodeId;
//!
//!     fn next(&mut self) -> Option<Self::Item> {
//!         while let Some(node) = self.stack.pop() {
//!             if self.discovery.visit(node).is_newly() {
//!                 for neighbor in self.graph.neighbors(node) {
//!                     if !self.discovery.was_visited(neighbor) {
//!                         self.stack.push(neighbor);
//!                     }
//!                 }
//!
//!                 return Some(node);
//!             }
//!         }
//!
//!         None
//!     }
//! }
//! ```
//!
//! we can avoid having to hold onto the borrow like so:
//!
//! ```
//! use graphs_core::{
//!     base::Base,
//!     id::NodeTypeId,
//!     neighbors::Neighbors,
//!     visit::{Visit, Visitor},
//!     walk::{Walk, Walker},
//! };
//!
//! struct Dfs<N: NodeTypeId, V: Visitor<N>> {
//!     stack: Vec<N>,
//!     discovered: V,
//! }
//!
//! type DfsOn<G> = Dfs<<G as Base>::NodeId, <G as Visit>::Visitor>;
//!
//! impl<G: Visit + Neighbors> Walker<G> for DfsOn<G> {
//!     type Item = G::NodeId;
//!
//!     fn walk_next(&mut self, graph: &G) -> Option<Self::Item> {
//!         while let Some(node) = self.stack.pop() {
//!             if self.discovered.visit(node) {
//!                 for neighbor in graph.neighbors(node) {
//!                     if !self.discovered.was_visited(neighbor) {
//!                         self.stack.push(neighbor);
//!                     }
//!                 }
//!
//!                 return Some(node);
//!             }
//!         }
//!
//!         None
//!     }
//! }
//!
//! type DfsWalk<'g, G> = Walk<'g, G, DfsOn<G>>;
//! ```

/// Represents *walkers*. See [module] documentation for more information.
///
/// [module]: self
pub trait Walker<C: ?Sized> {
    /// The type of items yielded by the walker.
    type Item;

    /// Advances the walker and returns the next item, if any.
    ///
    /// Returns [`None`] if there are no more items to return.
    fn walk_next(&mut self, context: &C) -> Option<Self::Item>;

    /// Converts the walker into [`Walk`] which contains the context and implements [`Iterator`].
    fn into_walk(self, context: &C) -> Walk<'_, C, Self>
    where
        Self: Sized,
    {
        Walk::new(context, self)
    }
}

impl<C: ?Sized, W: Walker<C> + ?Sized> Walker<C> for &mut W {
    type Item = W::Item;

    fn walk_next(&mut self, context: &C) -> Option<Self::Item> {
        (*self).walk_next(context)
    }
}

/// Represents iterators holding walkers and their relevant context.
///
/// The implementation of [`Iterator`] for this struct is as trivial as simply calling [`walk_next`]
/// on the contained walker and providing the contained context to it.
///
/// [`walk_next`]: Walker::walk_next
pub struct Walk<'c, C: ?Sized, W: Walker<C>> {
    /// The context that the walker operates on.
    context: &'c C,

    /// The walker that traverses the structure.
    walker: W,
}

impl<'c, C: ?Sized, W: Walker<C>> Walk<'c, C, W> {
    /// Constructs [`Self`] from the given context and walker.
    pub const fn new(context: &'c C, walker: W) -> Self {
        Self { context, walker }
    }

    /// Consumes [`Self`] and returns the contained walker,
    /// essentially *detaching* it from the context.
    pub fn detach(self) -> W {
        self.walker
    }
}

impl<C: ?Sized, W: Walker<C>> Iterator for Walk<'_, C, W> {
    type Item = W::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker.walk_next(self.context)
    }
}
