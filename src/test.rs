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
use std::error::Error;
use std::fmt;
use std::result::Result as RResult;
use std::str::FromStr;

use condition;
use error;
use state;


#[derive(PartialEq, Eq, Debug)]
pub struct TestCond {
    name: String,
}

impl From<&'static str> for TestCond {
    fn from(s: &str) -> Self {
        Self {name: s.to_owned()}
    }
}

impl condition::Condition for TestCond {
    type Issue = BTreeMap<&'static str, bool>;

    fn satisfied_by(&self, issue: &Self::Issue) -> bool {
        issue.get(self.name.as_str()).cloned().unwrap_or(false)
    }
}

impl FromStr for TestCond {
    type Err = TestCondParseError;

    fn from_str(s: &str) -> RResult<Self, Self::Err> {
        Ok(Self {name: s.to_owned()})
    }
}


#[derive(Default)]
pub struct TestCondFactory {}

impl condition::ConditionFactory<TestCond> for TestCondFactory {
    type Error = TestCondParseError;

    fn make_condition(
        &self,
        name: &str,
        neg: bool,
        val_op: Option<(condition::MatchOp, &str)>
    ) -> RResult<TestCond, TestCondParseError> {
        Ok(TestCond { name: name.to_owned() })
    }
}


#[derive(Debug, Default)]
pub struct TestCondParseError {
    inner: Option<error::Error>,
}

impl fmt::Display for TestCondParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> RResult<(), fmt::Error> {
        self.inner
            .as_ref()
            .map(|e| e.fmt(f))
            .unwrap_or(Ok(()))
    }
}

impl Error for TestCondParseError {}

impl From<error::Error> for TestCondParseError {
    fn from(e: error::Error) -> Self {
        Self { inner: Some(e) }
    }
}


pub type TestState = state::IssueState<TestCond>;

