use proc_macro2::Ident;
use syn::{
    Generics, Result, TraitBound,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::{Colon, Plus, Where},
};

pub type Bounds = Punctuated<TraitBound, Plus>;

pub struct Explicit {
    pub name: Ident,
    pub generics: Generics,
    pub bounds: Bounds,
}

impl Parse for Explicit {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name = input.parse()?;

        let mut generics: Generics = input.parse()?;

        // ensure the colon is present, ignoring it
        let _ = input.parse::<Colon>()?;

        let mut bounds = Punctuated::new();

        loop {
            if input.peek(Where) {
                break;
            }

            let bound = input.parse()?;

            bounds.push_value(bound);

            if input.peek(Where) || input.is_empty() {
                break;
            }

            let punctuation = input.parse()?;

            bounds.push_punct(punctuation);
        }

        generics.where_clause = input.parse()?;

        let explicit = Self {
            name,
            generics,
            bounds,
        };

        Ok(explicit)
    }
}

pub struct Implicit {
    pub name: Ident,
    pub generics: Generics,
}

impl Implicit {
    pub fn explicit(self, bounds: Bounds) -> Explicit {
        Explicit {
            name: self.name,
            generics: self.generics,
            bounds,
        }
    }

    pub fn explicit_with<F: FnOnce() -> Bounds>(self, function: F) -> Explicit {
        self.explicit(function())
    }
}

impl Parse for Implicit {
    fn parse(input: ParseStream) -> Result<Self> {
        let name = input.parse()?;

        let mut generics: Generics = input.parse()?;

        generics.where_clause = input.parse()?;

        let implicit = Self { name, generics };

        Ok(implicit)
    }
}
