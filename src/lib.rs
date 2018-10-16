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

//! # Issue states
//!
//! This library serves as a reference implementation for the concept of
//! "issue states": issues in an issue tracker are assigned an `IssueState`
//! each, based on `Condition`s associated with each state. The conditions
//! represent predicates on issues or their metadata.
//!
//! Users of this library will implement the trait `Condition`, which links
//! the user's issue-type (or, for example, a type representing an issue's
//! metadata) to the `IssueState`s provided by this library.
//!
//! Given some issue-states, an `IssueStateSet` may be constructed. This type
//! allows resolving a given issue's state, honouring relations between the
//! states contained in the set.
//!
//! `IssueState`s, and an `IssueStateSet`, may be constructed by the library's
//! user manually. However, this library also provides means for parsing an
//! `IssueStateSet` directly from a byte-stream. Currently, only the YAML format
//! is supported (if this library is compiled with support for `yaml-rust`
//! enabled).
//!

#[cfg(feature = "yaml-rust")]
extern crate yaml_rust;

pub mod condition;
pub mod error;
pub mod resolution;
pub mod state;

mod iter;

#[cfg(feature = "yaml-rust")]
pub mod yaml;

#[cfg(test)]
mod test;

// convenience exports
pub use error::Result;
pub use resolution::IssueStateSet;
pub use condition::Condition;
pub use state::IssueState;

