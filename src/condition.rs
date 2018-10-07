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

