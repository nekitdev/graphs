use proc_macro2::Span;
use syn::Ident;

pub type StaticStr = &'static str;

#[derive(Clone, Copy)]
pub struct Name {
    string: StaticStr,
}

impl Name {
    pub const fn new(string: StaticStr) -> Self {
        Self { string }
    }

    pub const fn get(self) -> StaticStr {
        self.string
    }

    pub const GRAPH_TYPE: Self = Self::new(stringify!(__G));

    pub fn ident(self) -> Ident {
        Ident::new(self.get(), Span::call_site())
    }
}
