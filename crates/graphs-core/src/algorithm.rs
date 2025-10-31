/// Represents algorithms on graphs `G`.
pub trait Algorithm<G> {
    /// The associated type for the algorithm output.
    type Output;

    /// Performs [`Self`] on the given graph `G`.
    fn perform(&mut self, graph: G) -> Self::Output;
}

impl<G, A: Algorithm<G>> Algorithm<G> for &mut A {
    type Output = A::Output;

    fn perform(&mut self, graph: G) -> Self::Output {
        (*self).perform(graph)
    }
}
