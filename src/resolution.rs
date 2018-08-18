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

//! State resolution facilities
//!
//! This module provides the `Resolvable` trait for resolution of a given
//! issue's state as well as types implementing it for issue state containers.
//!

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


/// Trait providing operation for resolving issues' states
///
/// Implementations of trait provide the reesolution of an issue's state. It is
/// generally assumed that the implementation encapsulates the states considered
/// for the resolution. For example, this may be implemented for containers of
/// issue states.
///
pub trait Resolvable<C>
    where C: state::Condition
{
    /// Resolve the state for a given issue
    ///
    /// Given an issue, this function will yield the state selected for it out
    /// of the issue states encapsulated in `self` --if any of the states is
    /// enabled for the issue.
    ///
    /// If no state is enabled for the given issue, this function will yield
    /// `None`.
    ///
    fn issue_state(&self, issue: &C::Issue) -> Result<Option<Arc<state::IssueState<C>>>>;
}




/// Set of issue states
///
/// This set of issue states is intended for the efficient computation of an
/// issue's state.
///
pub struct IssueStateSet<C>
    where C: state::Condition
{
    /// Container of states
    ///
    /// The states are kept in a linear sequence, ordered by dependency:
    /// an iterator over the slice will yield a state only after all its
    /// dependencies are yielded. Dependencies in this context are states
    /// which are extended or overridden by the yielded state.
    ///
    data: Box<[Arc<state::IssueState<C>>]>,
}


impl<C> IssueStateSet<C>
    where C: state::Condition
{
    /// Create an issue state set from a orderd set of issue states
    ///
    /// # Note:
    ///
    /// The set provided must be the (transitive) closure of all its elements
    /// regarding its relations to other sets: if a state is in the set, all
    /// states related to it must also be in the set. No explicit checking is
    /// performed to assert this property.
    ///
    pub fn from_set(mut states: collections::BTreeSet<Arc<state::IssueState<C>>>) -> Result<Self> {
        // We generate the state set by transferring states from the origin set
        // (`states`) to the result sequence (`data`) dependencies first.
        let mut data = Vec::default();
        while !states.is_empty() {
            let old_len = data.len();

            // We add all states for which no dependencies are left in the
            // origin set
            data.extend(states
                .iter()
                .filter(|state| !state
                    .relations
                    .iter()
                    .join_left(states.iter().map(|item| (item, ())))
                    .any(|item| item.1.is_some())
                )
                .map(Clone::clone));

            // Remove the states which are new in the target
            for state in data.split_at(old_len).1 {
                states.remove(state);
            }

            // If we did not find any state with no dependencies, there must be
            // a dependency cycle in the remaining origin set. We do this after
            // the removal for better reporting... eventually.
            if data.len() == old_len {
                return Err(Error::from(ErrorKind::CyclicDependency));
            }
        }

        Ok(Self {data: data.into_boxed_slice()})
    }
}


impl<C> Resolvable<C> for IssueStateSet<C>
    where C: state::Condition
{
    fn issue_state(&self, issue: &C::Issue) -> Result<Option<Arc<state::IssueState<C>>>> {
        let mut retval = None;
        let mut enabled_map = EnabledMap::default();

        // Since the data is nicely ordered in `data`, one liear pass over the
        // states is sufficient for selecting one for any given issue. We simply
        // determine the state foe each one as we go and keep the last of the
        // enabled states.
        for state in self.data.iter() {
            let enabled = state.conditions_satisfied(issue)
                && deps_enabled(&state, &enabled_map)?;
            enabled_map.insert(state.clone(), enabled);
            if enabled {
                retval = Some(state);
            }
        }

        Ok(retval.map(Clone::clone))
    }
}


/// Create an issue state set directly from a vector
///
/// # Warning
///
/// Within the vector, the states must appear ordered by dependency: all
/// dependencies of a state must appear before the state itself!
///
impl<C> From<state::IssueStateVec<C>> for IssueStateSet<C>
    where C: state::Condition
{
    fn from(states: Vec<Arc<state::IssueState<C>>>) -> Self {
        Self {data: states.into_boxed_slice()}
    }
}




#[cfg(test)]
mod tests {
    use super::*;
    use test::TestState;

    #[test]
    fn smoke() {
        let state1 : Arc<TestState> = state::IssueState::new("new".to_string()).into();

        let state2 : Arc<TestState> = {
            let mut tmp = state::IssueState::new("acknowledged".to_string());
            tmp.conditions = vec!["acked".into()];
            tmp.add_overridden([state1.clone()].into_iter().map(Clone::clone));
            tmp
        }.into();

        let state3 : Arc<TestState> = {
            let mut tmp = state::IssueState::new("assigned".to_string());
            tmp.conditions = vec!["assigned".into()];
            tmp.add_extended([state2.clone()].into_iter().map(Clone::clone));
            tmp
        }.into();

        let state4 : Arc<TestState> = {
            let mut tmp = state::IssueState::new("closed".to_string());
            tmp.conditions = vec!["closed".into()];
            tmp.add_overridden([state3.clone()].into_iter().map(Clone::clone));
            tmp
        }.into();

        let states = IssueStateSet::from_set({
            let mut set = collections::BTreeSet::new();
            set.insert(state1);
            set.insert(state2);
            set.insert(state3);
            set.insert(state4);
            set
        }).expect("Failed to create issue state set.");

        {
            let state = states
                .issue_state(&collections::BTreeMap::new())
                .expect("Failed to determine state.")
                .expect("Wrongly determined no state.");
            assert_eq!(state.name(), "new");
        }

        {
            let mut issue = collections::BTreeMap::new();
            issue.insert("acked", true);
            let state = states
                .issue_state(&issue)
                .expect("Failed to determine state.")
                .expect("Wrongly determined no state.");
            assert_eq!(state.name(), "acknowledged");
        }

        {
            let mut issue = collections::BTreeMap::new();
            issue.insert("assigned", true);
            let state = states
                .issue_state(&issue)
                .expect("Failed to determine state.")
                .expect("Wrongly determined no state.");
            assert_eq!(state.name(), "new");
        }

        {
            let mut issue = collections::BTreeMap::new();
            issue.insert("acked", true);
            issue.insert("assigned", true);
            let state = states
                .issue_state(&issue)
                .expect("Failed to determine state.")
                .expect("Wrongly determined no state.");
            assert_eq!(state.name(), "assigned");
        }

        {
            let mut issue = collections::BTreeMap::new();
            issue.insert("acked", true);
            issue.insert("closed", true);
            let state = states
                .issue_state(&issue)
                .expect("Failed to determine state.")
                .expect("Wrongly determined no state.");
            assert_eq!(state.name(), "closed");
        }
    }
}

