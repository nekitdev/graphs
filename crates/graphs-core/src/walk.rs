//! Walkers are iterators with context.

/// Represents walkers that can traverse some structure with the context `C`.
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
/// on the contained walker, providing the context to it.
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

impl<'c, C: ?Sized, W: Walker<C>> Iterator for Walk<'c, C, W> {
    type Item = W::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.walker.walk_next(self.context)
    }
}
