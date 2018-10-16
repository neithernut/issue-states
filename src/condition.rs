// Issue states
//
// Copyright (c) 2018 Julian Ganz
//
// MIT License
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//! Issue states and conditions
//!
//! This module provides the `Condition` trait which will usually be implemented
//! by the library's user.
//!

use std::error::Error as EError;
use std::result::Result as RResult;

use error::*;




/// Trait for issue metadata conditions
///
/// A `Condition` represents a predicate for an issue state: a function mapping
/// an issue to a boolean value indicating whether the condition is fulfilled or
/// not. It is generally assumed that a condition consists of "condition atoms",
/// which each specify a "singular" condition on a specific piece of metadata.
///
/// Whatever is used as type for conditions on metadata has to implement this
/// trait. It enables `IssueStates` to evaluate the condition. Additionally, the
/// `ConditionFactory` trait should be implemented in order to enable parsing
/// conditions from configuration files.
///
pub trait Condition {
    /// Type of the issue being evaluated
    ///
    /// Alternatively, some representation of the metadata may be used in place
    /// of the issue type.
    ///
    type Issue;

    /// Check whether the condition is satisfied by the issue provided
    ///
    fn satisfied_by(&self, issue: &Self::Issue) -> bool;
}




/// Match operators
///
/// These operators define how the piece of metadata queried from the issue is
/// compared to the literal provided with the conditon atom. The former is
/// considered the "left-hand value" while the latter is considered the
/// "right-hand value" in this context.
///
#[derive(Debug, PartialEq, Eq)]
pub enum MatchOp {
    /// Match if the values are evivalent
    Equivalence,
    /// Match if the left-hand value is lower than the right-hand value.
    LowerThan,
    /// Match if the left-hand value is greater than the right-hand value.
    GreaterThan,
    /// Match if the left-hand value is lower than the right-hand value or
    /// equal.
    LowerThanOrEqual,
    /// Match if the left-hand value is greater than the right-hand value or
    /// equal.
    GreaterThanOrEqual,
    /// Match if the left-hand value contains or is equal to the right-hand
    /// value.
    Contains,
}




/// Factory trait for conditions
///
/// This trait allows issue states parsers to create conditions from a string
/// representation. Implementers need not implement the actual parsing. Instead,
/// the function `make_condition()` will be supplied with the components of a
/// condition.
///
pub trait ConditionFactory<C>
    where C: Condition + Sized
{
    type Error : From<Error> + EError;

    /// Create a condition from bits and pieces
    ///
    /// The condition will be assembled from the "metadata identifier" (e.g. the
    /// name of the piece of metadata), a flag indicating whether the condition
    /// is negated or not and, optionally, the matching operator and a string
    /// representation of the right-hand side value.
    ///
    /// If the operator and value are not present, the resulting condition is
    /// expected to yield true if the piece of metadata denoted by the metadata
    /// identifier is present, e.g. non-null.
    ///
    fn make_condition(
        &self,
        name: &str,
        neg: bool,
        val_op: Option<(MatchOp, &str)>
    ) -> RResult<C, Self::Error>;

    /// Parse a condition directly from a string
    ///
    /// This function parses a `Condition` directly from a string using the
    /// `make_condition()` function.
    ///
    fn parse_condition(
        &self,
        string: &str,
    ) -> RResult<C, Self::Error> {
        parse_condition(string)
            .map_err(From::from)
            .and_then(|(name, neg, op_val)| self.make_condition(name, neg, op_val))
    }
}




/// Parse the bits of a condition atom
///
/// This method parses a condition atom. It returns the "metadata identifier"
/// (e.g. the name of the piece of metadata), a flag indicating whether the
/// condition is negated or not and, optionally, the matching operator and a
/// string representation of the right-hand side value.
///
/// The matching operator and value may be `None`. In this case, the condition
/// parsed is expected to check for the existence of a piece of metadata.
///
pub fn parse_condition(string: &str) -> Result<(&str, bool, Option<(MatchOp, &str)>)> {
    if let Some(pos) = string.find(|ref c| reserved_char(c)) {
        if pos == 0 {
            // The condition is either a negated existance (e.g. starts with
            // `!`) or invalid.
            let (neg, name) = string.split_at(1);
            return if neg == "!" && !name.contains(|ref c| reserved_char(c)) {
                Ok((name, true, None))
            } else {
                Err(Error::from(ErrorKind::ConditionParseError))
            }
        }

        let (name, mut op_val) = string.split_at(pos);
        let negated = op_val.starts_with('!');
        if negated {
            op_val = op_val.split_at(1).1;
        }
        Ok((name, negated, parse_op_val(op_val)?.into()))
    } else {
        // If the string representation does not contain any reserved
        // characters, this condition is the existance of the piece of metadata.
        Ok((string, false, None))
    }
}


/// Check whether a character is a reserved character
///
fn reserved_char(c: &char) -> bool {
    ['!', '=', '<', '>', '~'].contains(c)
}


/// Parse and extract the match operator and value from the compound
///
fn parse_op_val(string: &str) -> Result<(MatchOp, &str)> {
    let mut chars = string.chars();

    let (op, pos) = match chars.next() {
        Some('=') => (MatchOp::Equivalence, 1),
        Some('<') => match chars.next() {
            Some('=') => (MatchOp::LowerThanOrEqual, 2),
            _ => (MatchOp::LowerThan, 1),
        },
        Some('>') => match chars.next() {
            Some('=') => (MatchOp::GreaterThanOrEqual, 2),
            _ => (MatchOp::GreaterThan, 1),
        },
        Some('~') => (MatchOp::Contains, 1),
        _ => return Err(Error::from(ErrorKind::ConditionParseError)),
    };

    Ok((op, string.split_at(pos).1))
}




#[cfg(test)]
mod tests {
    use super::*;

    fn parse(string: &str) -> (&str, bool, Option<(MatchOp, &str)>) {
        parse_condition(string).expect("Failed to parse condition atom!")
    }

    #[test]
    fn smoke() {
        assert_eq!(parse("foo"), ("foo", false, None));
        assert_eq!(parse("!foo"), ("foo", true, None));
        assert_eq!(parse("foo=bar"), ("foo", false, Some((MatchOp::Equivalence, "bar"))));
        assert_eq!(parse("foo<bar"), ("foo", false, Some((MatchOp::LowerThan, "bar"))));
        assert_eq!(parse("foo>bar"), ("foo", false, Some((MatchOp::GreaterThan, "bar"))));
        assert_eq!(parse("foo<=bar"), ("foo", false, Some((MatchOp::LowerThanOrEqual, "bar"))));
        assert_eq!(parse("foo>=bar"), ("foo", false, Some((MatchOp::GreaterThanOrEqual, "bar"))));
        assert_eq!(parse("foo!~bar"), ("foo", true, Some((MatchOp::Contains, "bar"))));
        assert_eq!(parse("foo!=bar"), ("foo", true, Some((MatchOp::Equivalence, "bar"))));
        assert_eq!(parse("foo!<bar"), ("foo", true, Some((MatchOp::LowerThan, "bar"))));
        assert_eq!(parse("foo!>bar"), ("foo", true, Some((MatchOp::GreaterThan, "bar"))));
        assert_eq!(parse("foo!<=bar"), ("foo", true, Some((MatchOp::LowerThanOrEqual, "bar"))));
        assert_eq!(parse("foo!>=bar"), ("foo", true, Some((MatchOp::GreaterThanOrEqual, "bar"))));
        assert_eq!(parse("foo!~bar"), ("foo", true, Some((MatchOp::Contains, "bar"))));
    }
}
