use std::{fmt, io::Error as IoError};

use pest::{Parser, error::Error as PestError, iterators::Pair};
use thiserror::Error;

mod dot {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "dot.pest"]
    pub struct DotParser;
}

pub use self::dot::{DotParser, Rule};

pub type DotError = PestError<Rule>;
pub type DotPair<'a> = Pair<'a, Rule>;
pub type DotRules = Vec<Rule>;

#[derive(Debug, Error)]
pub struct ExpectedRule {
    expected: DotRules,
    found: Rule,
}

impl ExpectedRule {
    pub const fn new(expected: DotRules, found: Rule) -> Self {
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
    parent: DotPair<'a>,
    expected: DotRules,
}

impl<'a> MissingPair<'a> {
    pub const fn new(parent: DotPair<'a>, expected: DotRules) -> Self {
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
pub enum ParseError<'a> {
    ExpectedRule(ExpectedRule),
    MissingPair(MissingPair<'a>),
}

impl From<ExpectedRule> for ParseError<'_> {
    fn from(expected_rule: ExpectedRule) -> Self {
        Self::ExpectedRule(expected_rule)
    }
}

impl<'a> From<MissingPair<'a>> for ParseError<'a> {
    fn from(missing_pair: MissingPair<'a>) -> Self {
        Self::MissingPair(missing_pair)
    }
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ExpectedRule(expected_rule) => expected_rule.fmt(formatter),
            Self::MissingPair(missing_pair) => missing_pair.fmt(formatter),
        }
    }
}

#[derive(Debug, Error)]
pub enum FromFileError<'a> {
    File(IoError),
    Dot(DotError),
    Parse(ParseError<'a>),
}

impl From<IoError> for FromFileError<'_> {
    fn from(file: IoError) -> Self {
        Self::File(file)
    }
}

impl From<DotError> for FromFileError<'_> {
    fn from(dot: DotError) -> Self {
        Self::Dot(dot)
    }
}

impl<'a> From<ParseError<'a>> for FromFileError<'a> {
    fn from(parse: ParseError<'a>) -> Self {
        Self::Parse(parse)
    }
}

impl fmt::Display for FromFileError<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File(file) => file.fmt(formatter),
            Self::Dot(dot) => dot.fmt(formatter),
            Self::Parse(parse) => parse.fmt(formatter),
        }
    }
}

pub trait Parse<'a>: Sized {
    fn parse(input: DotPair<'a>) -> Result<Self, ParseError<'a>>;
}

pub const INPUT: &str = "expected input";
pub const INTERNAL: &str = "internal error during parsing";

pub fn parse_str<'a, T: Parse<'a>>(rule: Rule, input: &'a str) -> Result<T, DotError> {
    let mut pairs = DotParser::parse(rule, input)?;

    let pair = pairs.next().expect(INPUT);

    let parsed = T::parse(pair).expect(INTERNAL);

    Ok(parsed)
}
