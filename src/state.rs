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

use std::collections::BTreeMap;
use std::cmp::Ordering;
use std::sync::Arc;




/// Enumeration type for classificatoin of relations
///
pub enum StateRelation {
    Extends,
    Overrides
}




/// Trait for issue metadata conditions
///
/// Whatever is used as type for conditions on metadata has to implement this
/// trait. It enables `IssueStates` to evaluate the condition.
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




/// Representaiton of an issue state
///
pub struct IssueState<C>
    where C: Condition + Sized
{
    /// The name of the state
    name: String,
    /// Metadata conditions of the state
    pub conditions: Vec<C>,
    /// Relations to ther states
    pub relations: BTreeMap<Arc<IssueState<C>>, StateRelation>,
}


impl<C> IssueState<C>
    where C: Condition + Sized
{
    /// Create an issue state with a given name
    ///
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            conditions: Vec::new(),
            relations: BTreeMap::new(),
        }
    }

    /// Retrieve the name of the issue state
    ///
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Add states on which this state depends on
    ///
    pub fn add_extended<I>(&mut self, dependencies: I)
        where I: IntoIterator<Item = Arc<IssueState<C>>>
    {
        let entries = dependencies
            .into_iter()
            .map(|state| (state, StateRelation::Extends));
        self.relations.extend(entries)
    }

    /// Add states which will override this state
    ///
    pub fn add_overridden<I>(&mut self, overridden_by: I)
        where I: IntoIterator<Item = Arc<IssueState<C>>>
    {
        let entries = overridden_by
            .into_iter()
            .map(|state| (state, StateRelation::Overrides));
        self.relations.extend(entries)
    }

    /// Check whether all conditions of the state are satisfied for an issue
    ///
    /// # Note:
    ///
    /// Conditions inherited from states extended by this state are not
    /// considered. Thus, this function alone can not be used for assessing
    /// whether the state is enabled or not.
    ///
    pub fn conditions_satisfied(&self, issue: &C::Issue) -> bool {
        self.conditions.iter().all(|c| c.satisfied_by(issue))
    }
}


impl<C> PartialEq for IssueState<C>
    where C: Condition
{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}


impl<C> Eq for IssueState<C>
    where C: Condition
{}


impl<C> PartialOrd for IssueState<C>
    where C: Condition
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}


impl<C> Ord for IssueState<C>
    where C: Condition
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}

