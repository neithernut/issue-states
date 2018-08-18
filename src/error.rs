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

//! Obligatory error module
//!
//! This module provides an `Error` and a `Result` type for this library.

use std::fmt;
use std::error::Error as EError;
use std::result::Result as RResult;




/// Kinds of errors
///
pub enum ErrorKind {
    /// A cyclic dependency was dected among a set of states
    ///
    /// Cyclic dependencies among issue states are forbidden.
    ///
    CyclicDependency,
    /// An issue's dependency could not be resolved
    ///
    DependencyError,
}




/// Error type for use within the library
///
pub struct Error {
    kind: ErrorKind
}


impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Self {kind: kind}
    }
}


impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.kind {
            ErrorKind::CyclicDependency => f.write_str("dependency cycle detected"),
            ErrorKind::DependencyError => f.write_str("dependency resolution error"),
        }
    }
}


impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}


impl EError for Error {
    fn description(&self) -> &str {
        "Resolution failed"
    }
}




pub type Result<T> = RResult<T, Error>;

