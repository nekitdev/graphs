cfg_select! {
    feature = "std" => {
        use std::fmt;
    }
    _ => {
        use core::fmt;

        use alloc::vec::Vec;
    }
}

use pest::{Parser, error::Error as PestError, iterators::Pair};
use thiserror::Error;
use trait_aliases::trait_aliases;

mod dot {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "dot.pest"]
    pub struct DotParser;
}

pub use self::dot::{DotParser, Rule};

pub type ParseErrorInner = PestError<Rule>;
pub type ParsePair<'a> = Pair<'a, Rule>;
pub type ParseRules = Vec<Rule>;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct ParseError {
    #[from]
    pub inner: ParseErrorInner,
}

impl ParseError {
    pub fn get(self) -> ParseErrorInner {
        self.inner
    }
}

#[derive(Debug, Error)]
pub struct ExpectedRule {
    expected: ParseRules,
    found: Rule,
}

impl ExpectedRule {
    pub const fn new(expected: ParseRules, found: Rule) -> Self {
        Self { expected, found }
    }
}

const DELIMITER: &str = ", ";

impl fmt::Display for ExpectedRule {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let displayed: Vec<_> = self
            .expected
            .iter()
            .map(|rule| format!("`{rule:?}`"))
            .collect();

        let expected = displayed.join(DELIMITER);

        let found = self.found;

        write!(
            formatter,
            "expected one of {expected}, but `{found:?}` found"
        )
    }
}

#[derive(Debug, Error)]
pub struct MissingPair<'a> {
    parent: ParsePair<'a>,
    expected: ParseRules,
}

impl<'a> MissingPair<'a> {
    pub const fn new(parent: ParsePair<'a>, expected: ParseRules) -> Self {
        Self { parent, expected }
    }
}

impl fmt::Display for MissingPair<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let displayed: Vec<_> = self
            .expected
            .iter()
            .map(|rule| format!("`{rule:?}`"))
            .collect();

        let expected = displayed.join(DELIMITER);

        let parent = self.parent.as_str();

        write!(
            formatter,
            "the pair {parent} terminates early; expected one of {expected}"
        )
    }
}

#[derive(Debug, Error)]
pub enum InternalError<'a> {
    ExpectedRule(ExpectedRule),
    MissingPair(MissingPair<'a>),
}

impl From<ExpectedRule> for InternalError<'_> {
    fn from(expected_rule: ExpectedRule) -> Self {
        Self::ExpectedRule(expected_rule)
    }
}

impl<'a> From<MissingPair<'a>> for InternalError<'a> {
    fn from(missing_pair: MissingPair<'a>) -> Self {
        Self::MissingPair(missing_pair)
    }
}

impl fmt::Display for InternalError<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedRule(expected_rule) => expected_rule.fmt(formatter),
            Self::MissingPair(missing_pair) => missing_pair.fmt(formatter),
        }
    }
}

pub trait Parse<'a>: Sized {
    fn parse(input: ParsePair<'a>) -> Result<Self, InternalError<'a>>;
}

pub trait Ruled {
    const RULE: Rule;
}

pub const INPUT: &str = "expected input";
pub const INTERNAL: &str = "internal error during parsing";

#[inline(never)]
#[cold]
#[track_caller]
fn fail(error: InternalError<'_>) -> ! {
    panic!("{INTERNAL}: {error}")
}

pub trait ParseStr<'a>: Parse<'a> + Ruled {
    fn parse_str(string: &'a str) -> Result<Self, ParseError> {
        let mut pairs = DotParser::parse(Self::RULE, string)?;

        let pair = pairs.next().expect(INPUT);

        match Self::parse(pair) {
            Ok(parsed) => Ok(parsed),
            Err(error) => fail(error),
        }
    }
}

impl<'a, T: Parse<'a> + Ruled> ParseStr<'a> for T {}

trait_aliases! {
    #[trait_alias(T)]
    pub trait ParseOwned = for<'a> ParseStr<'a>;
}

#[cfg(feature = "std")]
mod from_file {
    use std::{fs::read_to_string, io::Error as IoError, path::Path};

    use thiserror::Error;

    use super::{ParseError, ParseOwned};

    #[derive(Debug, Error)]
    #[error(transparent)]
    pub enum FromFileError {
        File(#[from] IoError),
        Parse(#[from] ParseError),
    }

    pub trait FromFile: ParseOwned {
        fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, FromFileError> {
            let string = read_to_string(path)?;

            let parsed = Self::parse_str(string.as_str())?;

            Ok(parsed)
        }
    }

    impl<T: ParseOwned> FromFile for T {}
}

#[cfg(feature = "std")]
pub use from_file::{FromFile, FromFileError};

#[cfg(feature = "proc-macro")]
mod error {
    use core::fmt::Display;

    use proc_macro2::Span;
    use syn::Error;

    use super::ParseError;

    fn call_site_error<E: Display>(error: E) -> Error {
        Error::new(Span::call_site(), error)
    }

    impl From<ParseError> for Error {
        fn from(error: ParseError) -> Self {
            call_site_error(error)
        }
    }

    #[cfg(feature = "std")]
    use super::FromFileError;

    #[cfg(feature = "std")]
    impl From<FromFileError> for Error {
        fn from(error: FromFileError) -> Self {
            match error {
                FromFileError::File(file) => call_site_error(file),
                FromFileError::Parse(parse) => call_site_error(parse),
            }
        }
    }
}
