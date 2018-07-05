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

use std::cmp::Ordering;




/// Iterator for performing a left join
///
/// This iterator wraps two iterators, yielding key-value-tuples with
/// potentially different value types. This iterator yields tuples of values
/// from both iterators, matched by key. These include all values from the
/// left iterator, with the optional value found in the second for the
/// respective key. If no such value is found, a `None` value is returned
/// instead for the value.
///
/// # Note
///
/// This iterator assumes that both iterators yield elements sorted by the key,
/// with each key being unique within the respective list.
///
pub struct LeftJoin<L, R, K, U, V>
    where L: Iterator<Item = (K, U)>,
          R: Iterator<Item = (K, V)>,
          K: Ord
{
    left: L,
    right: R,
    buf: Option<R::Item>,
}


impl<L, R, K, U, V> LeftJoin<L, R, K, U, V>
    where L: Iterator<Item = (K, U)>,
          R: Iterator<Item = (K, V)>,
          K: Ord
{
    /// Create a new `LeftJoin`
    ///
    pub fn new(left: L, right: R) -> Self {
        Self {left: left, right: right, buf: None}
    }

    /// Get the next right element for a given "left" key
    ///
    fn next_right(&mut self, key: &K) -> Option<V> {
        let mut buf = self.buf.take().or_else(|| self.right.next());
        loop {
            return match buf.as_ref().map(|item| item.0.cmp(key)) {
                Some(Ordering::Less)  => { buf = self.right.next(); continue },
                Some(Ordering::Equal) => buf.map(|item| item.1),
                _                     => { self.buf = buf; None },
            };
        }
    }
}


impl<L, R, K, U, V> Iterator for LeftJoin<L, R, K, U, V>
    where L: Iterator<Item = (K, U)>,
          R: Iterator<Item = (K, V)>,
          K: Ord
{
    type Item = (U, Option<V>);

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.left.next();
        item.map(|item| {
            let value = self.next_right(&item.0);
            (item.1, value)
        })
    }
}




/// Convenience trait for creating a left join
///
pub trait LeftJoinable<L, R, K, U, V>
    where L: Iterator<Item = (K, U)>,
          R: Iterator<Item = (K, V)>,
          K: Ord
{
    fn join_left(self, right: R) -> LeftJoin<L, R, K, U, V>;
}


impl<L, R, K, U, V> LeftJoinable<L, R, K, U, V> for L
    where L: Iterator<Item = (K, U)>,
          R: Iterator<Item = (K, V)>,
          K: Ord
{
    fn join_left(self, right: R) -> LeftJoin<L, R, K, U, V> {
        LeftJoin::new(self, right)
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoke() {
        let left = vec![
            (1, "a"),
            (3, "b"),
            (4, "c"),
            (7, "d"),
            (8, "e")
        ];
        let right = vec![
            (2, "x"),
            (3, "u"),
            (4, "v"),
            (5, "x"),
            (6, "x"),
            (7, "w")
        ];

        let mut res = LeftJoin::new(left.into_iter(), right.into_iter());
        assert_eq!(res.next(), Some(("a", None)));
        assert_eq!(res.next(), Some(("b", Some("u"))));
        assert_eq!(res.next(), Some(("c", Some("v"))));
        assert_eq!(res.next(), Some(("d", Some("w"))));
        assert_eq!(res.next(), Some(("e", None)));
        assert_eq!(res.next(), None)
    }
}

