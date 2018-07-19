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

use std::error::Error;
use std::result::Result as RResult;
use std::sync::Arc;
use yaml_rust::{parser, scanner};

use state;
use resolution::IssueStateSet;




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
pub fn parse_issue_states<R, C, F, E>(
    parser: &mut parser::Parser<R>,
    cond_parse: F
) -> ParseResult<IssueStateSet<C>>
    where R: Iterator<Item = char>,
          C: state::Condition + Sized,
          F: FnMut(&str) -> RResult<C, E>,
          E: Error
{
    parse_issue_state_vec(parser, cond_parse).map(From::from)
}


/// Actual implementation of the parsing
///
/// We split the implementation in order to enable proper testing
///
fn parse_issue_state_vec<R, C, F, E>(
    parser: &mut parser::Parser<R>,
    mut cond_parse: F
) -> ParseResult<state::IssueStateVec<C>>
    where R: Iterator<Item = char>,
          C: state::Condition + Sized,
          F: FnMut(&str) -> RResult<C, E>,
          E: Error
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
            (parser::Event::MappingStart(_), _) => parse_issue_state_map(parser, &retval, &mut cond_parse)?,
            (_, marker) => return Err(scanner::ScanError::new(
                marker,
                "Expected issue state as either map or scalar"
            )),
        });

        retval.push(state);
    }

    Ok(retval)
}


/// Function for parsing an issue state represented as a map
///
fn parse_issue_state_map<R, C, F, E>(
    parser: &mut parser::Parser<R>,
    existing_states: &state::IssueStateVec<C>,
    cond_parse: &mut F
) -> ParseResult<state::IssueState<C>>
    where R: Iterator<Item = char>,
          C: state::Condition + Sized,
          F: FnMut(&str) -> RResult<C, E>,
          E: Error
{
    // TODO: implement
    Ok(state::IssueState::new(Default::default()))
}




#[cfg(test)]
mod tests {
    use super::*;
    use test::TestCond;

    use std::str::FromStr;

    // Convenience function for encapsulating boilerplate for each test
    //
    fn parse(s: &str) -> state::IssueStateVec<TestCond> {
        let mut parser = parser::Parser::new(s.chars());
        parse_issue_state_vec(&mut parser, FromStr::from_str)
            .expect("Failed to parse document")
    }

    #[test]
    fn empty_stream() {
        let result = parse("");
        assert!(result.is_empty());
    }

    #[test]
    fn single_state1() {
        let result = parse("  - foobar");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name(), "foobar");
    }

    #[test]
    fn single_state2() {
        let result = parse("---\n  - foobar\n...");
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name(), "foobar");
    }
}
