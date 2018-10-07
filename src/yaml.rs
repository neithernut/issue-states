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

//! Parse issue states from a YAML document using yaml-rust
//!
//! The primary function for parsing issue states is `parse_issue_states()`. The
//! states are expected to appear in a sequence. Each node within the sequence
//! must be either the name of a state or a mapping containing:
//! * a "name" entry denoting the name of the state,
//! * an optional "conditions" entry containing conditions, as a sequence of
//!   strings,
//! * an optional "overrides" entry containing a sequence of state names
//!   apprearing _prior_ to the current issue state in the toplevel sequence,
//!   and
//! * an optional "extends" entry containing a sequence of state names
//!   apprearing _prior_ to the current issue state in the toplevel sequence.
//!

use std::result::Result as RResult;
use std::sync::Arc;
use yaml_rust::{parser, scanner};

use condition;
use resolution::IssueStateSet;
use state;




/// Parser specific result type
///
pub type ParseResult<T> = RResult<T, scanner::ScanError>;


/// Parse issue states from a YAML document or stream
///
/// The function expects the `parser` used for parsing the document as well as a
/// function `cond_parse` for parsing the user-provided condition type. In the
/// most simple case, the caller may pass `FromStr::from_str` for `cond_parse`.
///
/// Both `StreamStart` and `DocumentStart` events will be skipped. Afterwrds,
/// the function will expect a sequence of nodes representing an issue state
/// each, as specified elsewhere.
///
/// # Note:
///
/// After the parser reached the ned of the sequence, the function will return.
/// Consuming more events until the end of the document or stream and generating
/// errors if any unexpected events are occured is the caller's responsibility.
///
pub fn parse_issue_states<R, C, F>(
    parser: &mut parser::Parser<R>,
    cond_factory: F
) -> ParseResult<IssueStateSet<C>>
    where R: Iterator<Item = char>,
          C: condition::Condition + Sized,
          F: condition::ConditionFactory<C>,
{
    // Skip the beginning of the document
    while match parser.peek()? {
        (parser::Event::StreamStart, _) => true,
        (parser::Event::DocumentStart, _) => true,
        _ => false,
    } { parser.next()?; }

    // Identify the start of the sequence
    match parser.peek()? {
        (parser::Event::StreamEnd, _) => return Ok(Default::default()),
        (parser::Event::DocumentEnd, _) => return Ok(Default::default()),
        (parser::Event::SequenceStart(_), _) => {},
        (_, marker) => return Err(scanner::ScanError::new(*marker, "Expected sequence of issue states")),
    };

    // Extract the SequenceStart event
    parser.next()?;

    let mut retval = state::IssueStateVec::default();

    // Parse individual issue states as items of the sequence
    loop {
        let state = Arc::new(match parser.next()? {
            (parser::Event::SequenceEnd, _) => break, // We hit the end of the sequence
            (parser::Event::Scalar(name, _, _, _), _) => state::IssueState::new(name),
            (parser::Event::MappingStart(_), _) => parse_issue_state_map(parser, &retval, &cond_factory)?,
            (_, marker) => return Err(scanner::ScanError::new(
                marker,
                "Expected issue state as either map or scalar"
            )),
        });

        retval.push(state);
    }

    Ok(retval.into())
}


/// Function for parsing an issue state represented as a map
///
fn parse_issue_state_map<R, C, F>(
    parser: &mut parser::Parser<R>,
    existing_states: &state::IssueStateVec<C>,
    cond_factory: &F
) -> ParseResult<state::IssueState<C>>
    where R: Iterator<Item = char>,
          C: condition::Condition + Sized,
          F: condition::ConditionFactory<C>,
{
    let mut name = Default::default();
    let mut conditions = Vec::default();
    let mut relations = state::StateRelations::default();

    loop {
        // Try to extract the key of the entry
        let (key, marker) = match parser.next()? {
            (parser::Event::MappingEnd, _) => break, // We hit the end of the map
            (parser::Event::Scalar(key, _, _, _), marker) => (key, marker),
            (_, marker) => return Err(scanner::ScanError::new(marker, "Expected scalar key")),
        };

        // Identify the entry and carry out the associated action
        match key.as_str() {
            "name" => match parser.next()? {
                (parser::Event::Scalar(value, _, _, _), _) => name = value,
                (_, marker) => return Err(scanner::ScanError::new(
                    marker,
                    "Expected state name as scalar")
                ),
            },
            "conditions" => for item in StringIter::new(parser) {
                let (cond, marker) = item?;
                conditions.push(cond_factory.parse_condition(cond.as_str()).map_err(|err| {
                    let s = err.to_string();
                    scanner::ScanError::new(
                        marker,
                        s.as_str()
                    )
                })?);
            }
            "overrides" => parse_state_relations(
                &mut relations,
                parser,
                existing_states,
                state::StateRelation::Overrides
            )?,
            "extends" => parse_state_relations(
                &mut relations,
                parser,
                existing_states,
                state::StateRelation::Extends
            )?,
            _ => return Err(scanner::ScanError::new(
                marker,
                "Expected either 'name', 'conditions', 'overrides' or 'extends'"
            )),
        }
    }

    let mut retval = state::IssueState::new(name);
    retval.conditions = conditions;
    retval.relations = relations;
    Ok(retval)
}


/// Function for parsing relations from a sequence of scalars
///
fn parse_state_relations<R, C>(
    relations: &mut state::StateRelations<C>,
    parser: &mut parser::Parser<R>,
    existing_states: &state::IssueStateVec<C>,
    relation: state::StateRelation
) -> ParseResult<()>
    where R: Iterator<Item = char>,
          C: condition::Condition + Sized,
{
    for item in StringIter::new(parser) {
        let (name, marker) = item?;
        let state = existing_states
            .iter()
            .find(|s| *s.name() == name)
            .map(Clone::clone)
            .ok_or_else(|| scanner::ScanError::new(
                marker,
                "Unknown state"
            ))?;
        relations.insert(state, relation.clone());
    }
    Ok(())
}


/// Iterator for iterating over the scalars in a sequence
///
/// This iterator allows convenient iteration over a sequence, assuming that the
/// sequence consists only of scalars. If a non-scalar is encountered, this
/// iterator will yield an error.
///
/// Alternative to a sequence of scalars, this iterator allows the convenient
/// view on a single scalar as if it were a sequence containing only a single
/// (scalar) item.
///
struct StringIter<'p, R>
    where R: Iterator<Item = char> + 'p
{
    parser: &'p mut parser::Parser<R>,
    state: SequenceParseState,
}

impl<'p, R> StringIter<'p, R>
    where R: Iterator<Item = char> + 'p
{
    /// Create a new StringIter
    ///
    /// # Note:
    ///
    /// The `parser` should point at the beginning of the list, e.g. the `Event`
    /// yielded by the parser should be either a `SequenceStart` or, in the
    /// special case, the `Scalar` making up the list. Use this at the point
    /// where you expect a list.
    ///
    fn new(parser: &'p mut parser::Parser<R>) -> Self {
        Self {parser: parser, state: SequenceParseState::Start}
    }

    /// Given that we are within a sequence, extract the next item
    ///
    fn in_sequence(&mut self) -> Option<<Self as Iterator>::Item> {
        match self.parser.next() {
            Ok((parser::Event::Scalar(s, _, _, _), m)) => Some(Ok((s, m))),
            Ok((parser::Event::SequenceEnd, _)) => {
                self.state = SequenceParseState::End;
                None
            }
            Ok((_, marker)) => {
                self.state = SequenceParseState::End;
                Some(Err(scanner::ScanError::new(marker, "Expected scalar")))
            }
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'p, R> Iterator for StringIter<'p, R>
    where R: Iterator<Item = char> + 'p
{
    type Item = ParseResult<(String, scanner::Marker)>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            // The iterator is fresh. Figure out whether we have an actual
            // sequence or a single item.
            SequenceParseState::Start => match self.parser.next() {
                Ok((parser::Event::SequenceStart(_), _)) => {
                    self.state = SequenceParseState::Sequence;
                    self.in_sequence()
                }
                Ok((parser::Event::Scalar(s, _, _, _), m)) => {
                    self.state = SequenceParseState::End;
                    Some(Ok((s, m)))
                },
                Ok((_, marker)) => {
                    self.state = SequenceParseState::End;
                    Some(Err(scanner::ScanError::new(
                        marker,
                        "Expected scalar or list of scalars"
                    )))
                },
                Err(err) => Some(Err(err)),
            },
            SequenceParseState::Sequence => self.in_sequence(),
            // We reached the end of the sequence.
            SequenceParseState::End => None,
        }
    }
}


/// Type representing the internal state of a `StringIter`
enum SequenceParseState {
    Start,
    Sequence,
    End,
}




#[cfg(test)]
mod tests {
    use super::*;
    use test::{TestCond, TestCondFactory};

    // Convenience function for encapsulating boilerplate for each test
    //
    fn parse(s: &str) -> IssueStateSet<TestCond> {
        let mut parser = parser::Parser::new(s.chars());
        parse_issue_states(&mut parser, TestCondFactory::default())
            .expect("Failed to parse document")
    }

    #[test]
    fn empty_stream() {
        let result = parse("");
        assert_eq!(result.iter().count(), 0);
    }

    #[test]
    fn single_state1() {
        let result = parse("  - foobar");
        let mut iter = result.iter();

        let state = iter
            .next()
            .expect("Parse result does not contain expected state.");
        assert_eq!(state.name(), "foobar");

        assert!(iter.next().is_none());
    }

    #[test]
    fn single_state2() {
        let result = parse("---\n  - foobar\n...");
        let mut iter = result.iter();

        let state = iter
            .next()
            .expect("Parse result does not contain expected state.");
        assert_eq!(state.name(), "foobar");

        assert!(iter.next().is_none());
    }

    #[test]
    fn single_with_conditions() {
        let result = parse("---
  - name: foobar
    conditions: [foo, bar]
...");
        let mut iter = result.iter();

        let state = iter
            .next()
            .expect("Parse result does not contain expected state.");
        assert_eq!(state.name(), "foobar");
        assert_eq!(state.conditions, vec!["foo".into(), "bar".into()]);

        assert!(iter.next().is_none());
    }

    #[test]
    fn multiple_with_conditions() {
        let result = parse("---
  - new
  - name: acknowledged
    conditions: acked
    overrides: new
  - name: assigned
    conditions: assigned
    extends: acknowledged
  - name: closed
    conditions: closed
    overrides: [new, acknowledged, assigned]
...");

        let mut iter = result.iter();

        let state1 = iter
            .next()
            .expect("Parse result does not contain expected state.");
        assert_eq!(state1.name(), "new");

        let state2 = iter
            .next()
            .expect("Parse result does not contain expected state.");
        assert_eq!(state2.name(), "acknowledged");
        assert_eq!(state2.conditions, vec!["acked".into()]);
        assert_eq!(state2.relations.get(state1), Some(&state::StateRelation::Overrides));

        let state3 = iter
            .next()
            .expect("Parse result does not contain expected state.");
        assert_eq!(state3.name(), "assigned");
        assert_eq!(state3.conditions, vec!["assigned".into()]);
        assert_eq!(state3.relations.get(state2), Some(&state::StateRelation::Extends));

        let state4 = iter
            .next()
            .expect("Parse result does not contain expected state.");
        assert_eq!(state4.name(), "closed");
        assert_eq!(state4.conditions, vec!["closed".into()]);
        assert_eq!(state4.relations.get(state1), Some(&state::StateRelation::Overrides));
        assert_eq!(state4.relations.get(state2), Some(&state::StateRelation::Overrides));
        assert_eq!(state4.relations.get(state3), Some(&state::StateRelation::Overrides));

        assert!(iter.next().is_none());
    }
}
