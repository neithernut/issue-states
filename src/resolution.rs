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

use std::sync::Arc;
use std::collections;

use error::*;
use iter::LeftJoinable;
use state;




/// Map for tracking enabled and disabled states
///
type EnabledMap<C> = collections::BTreeMap<Arc<state::IssueState<C>>, bool>;


/// Check whether the dependencies for an issue's state allow it to be enabled
///
/// For example, an issue state may only be enabled if all the states it extends
/// are enabled. The computation is done purely based on a given `EnabledMap`,
/// e.g. this function does not recurse into extended states.
///
/// This function may be used for implementing efficient computation of an
/// issue's state.
///
fn deps_enabled<C>(state: &state::IssueState<C>, map: &EnabledMap<C>) -> Result<bool>
    where C: state::Condition
{
    state
        .relations
        .iter()
        .join_left(map.iter())
        .filter_map(|item| match item.0 {
            state::StateRelation::Extends   => Some(item.1),
            state::StateRelation::Overrides => None,
        })
        .fold(Some(true), |state, val| if let (Some(s), Some(v)) = (state, val) {
            Some(s && *v)
        } else {
            None
        })
        .ok_or_else(|| Error::from(ErrorKind::DependencyError))
        // TODO: replace with try_fold()
}


/// Trait providing operation for resolving issue states
///
pub trait Resolvable<C>
    where C: state::Condition
{
    /// Resolve states for a given issue
    ///
    fn issue_state(&self, issue: &C::Issue) -> Result<Option<Arc<state::IssueState<C>>>>;
}

