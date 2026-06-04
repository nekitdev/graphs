use syn::{
    Ident,
    visit::{Visit, visit_ident},
};

use crate::{context::Context, parse::Explicit};

pub struct Checker<'g, 'c> {
    graph: &'g Ident,
    context: &'c Context,
}

impl<'g, 'c> Checker<'g, 'c> {
    pub const fn new(graph: &'g Ident, context: &'c Context) -> Self {
        Self { graph, context }
    }
}

impl Checker<'_, '_> {
    pub fn message(&self) -> String {
        format!(
            "identifier `{graph}` is reserved for assertions",
            graph = self.graph
        )
    }

    pub fn visit(&self, identifier: &Ident) {
        if identifier == self.graph {
            self.context.error_spanned_by(identifier, self.message());
        }
    }

    pub fn visit_explicit(&mut self, explicit: &Explicit) {
        self.visit_ident(&explicit.name);

        self.visit_generics(&explicit.generics);

        explicit
            .bounds
            .iter()
            .for_each(|bound| self.visit_trait_bound(bound));
    }
}

impl<'a> Visit<'a> for Checker<'_, '_> {
    fn visit_ident(&mut self, identifier: &'a Ident) {
        self.visit(identifier);

        visit_ident(self, identifier);
    }
}
