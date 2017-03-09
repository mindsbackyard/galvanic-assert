/* Copyright 2017 Christopher Bacher
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use super::super::*;

/// Takes a list of matchers for the same type combines them conjunctively.
#[macro_export]
macro_rules! all_of {
    ( $matcher: expr ) => {
        All::of($matcher)
    };
    ( $matcher: expr, $($matchers: expr),* ) => {
        All::of($matcher)$(.and($matchers))*
    };
}

/// A `Matcher` struct which joins multiple `Matcher`s conjunctively.
///
/// Use `of()` to create a new `Matcher` and `and()` to add further `Matcher`s.
pub struct All<'a, T:'a> {
    matcher: Box<Matcher<&'a T> + 'a>,
    next: Option<Box<All<'a,T>>>
}

impl<'a,T:'a> All<'a, T> {
    /// Creates a new conjunctive `Matcher` starting with the given `Matcher`.
    pub fn of<M>(matcher: M) -> All<'a,T>
    where M: Matcher<&'a T> + 'a, T: 'a {
        All {
            matcher: Box::new(matcher),
            next: None
        }
    }

    /// Adds the given `Matcher` conjunctively.
    pub fn and<M>(self, matcher: M) -> All<'a,T>
    where M: Matcher<&'a T> + 'a, T: 'a {
        All {
            matcher: Box::new(matcher),
            next: Some(Box::new(self))
        }
    }
}

impl<'a,T:'a> Matcher<&'a T> for All<'a,T> {
    fn check(&self, actual: &'a T) -> MatchResult {
        match self.matcher.check(actual) {
            x@MatchResult::Matched {..} => {
                match self.next {
                    None => x,
                    Some(ref next) => next.check(actual)
                }
            },
            x@MatchResult::Failed {..} => x
        }
    }
}

/// Takes a list of matchers for the same type combines them disjunctively.
#[macro_export]
macro_rules! any_of {
    ( $matcher: expr ) => {
        Any::of($matcher)
    };
    ( $matcher: expr, $($matchers: expr),* ) => {
        Any::of($matcher)$(.or($matchers))*
    };
}

/// A `Matcher` struct which joins multiple `Matcher`s disjunctively.
///
/// Use `of()` to create a new `Matcher` and `or()` to add further `Matcher`s.
pub struct Any<'a, T:'a> {
    matcher: Box<Matcher<&'a T> + 'a>,
    next: Option<Box<Any<'a,T>>>
}

impl<'a,T:'a> Any<'a, T> {
    /// Creates a new conjunctive `Matcher` starting with the given `Matcher`.
    pub fn of<M>(matcher: M) -> Any<'a,T>
    where M: Matcher<&'a T> + 'a, T: 'a {
        Any {
            matcher: Box::new(matcher),
            next: None
        }
    }

    /// Adds the given `Matcher` disjunctively.
    pub fn or<M>(self, matcher: M) -> Any<'a,T>
    where M: Matcher<&'a T> + 'a, T:'a {
        Any {
            matcher: Box::new(matcher),
            next: Some(Box::new(self))
        }
    }
}

impl<'a,T:'a> Matcher<&'a T> for Any<'a,T> {
    fn check(&self, actual: &'a T) -> MatchResult {
        match self.matcher.check(actual) {
            MatchResult::Matched {..} => MatchResult::Matched { name: "any_of".to_owned() },
            x@MatchResult::Failed {..} => match self.next {
                None => x,
                Some(ref next) => next.check(actual)
            }
        }
    }
}
