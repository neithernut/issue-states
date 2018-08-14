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

impl state::Condition for TestCond {
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


#[derive(Debug)]
pub struct TestCondParseError {
}

impl fmt::Display for TestCondParseError {
    fn fmt(&self, _: &mut fmt::Formatter) -> RResult<(), fmt::Error> {
        Ok(())
    }
}

impl Error for TestCondParseError {}


pub type TestState = state::IssueState<TestCond>;

