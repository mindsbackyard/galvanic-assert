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
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), all_of![gt(0), lt(5), not(eq(3))]);
/// # }
#[macro_export]
macro_rules! all_of {
    ( $matcher: expr ) => {
        Box::new(All::of($matcher))
    };
    ( $matcher: expr, $($matchers: expr),* ) => {
        Box::new(All::of($matcher)$(.and($matchers))*)
    };
}

/// A `Matcher` struct which joins multiple `Matcher`s conjunctively.
///
/// Use `of()` to create a new `Matcher` and `and()` to add further `Matcher`s.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), All::of(gt(0)).and(lt(5)).and(not(eq(3))));
/// # }
pub struct All<T> {
    pub matcher: Box<Matcher<T>>,
    pub next: Option<Box<All<T>>>
}

impl<T> All<T> {
    /// Creates a new conjunctive `Matcher` starting with the given `Matcher`.
    pub fn of(matcher: Box<Matcher<T>>) -> All<T> {
        All {
            matcher: matcher,
            next: None
        }
    }

    /// Adds the given `Matcher` conjunctively.
    pub fn and(self, matcher: Box<Matcher<T>>) -> All<T> {
        All {
            matcher: matcher,
            next: Some(Box::new(self))
        }
    }
}

impl<T> Matcher<T> for All<T> {
    fn check(&self, actual: &T) -> MatchResult {
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
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), any_of![lt(0), gt(5), not(eq(3))]);
/// # }
#[macro_export]
macro_rules! any_of {
    ( $matcher: expr ) => {
        Box::new(Any::of($matcher))
    };
    ( $matcher: expr, $($matchers: expr),* ) => {
        Box::new(Any::of($matcher)$(.or($matchers))*)
    };
}

/// A `Matcher` struct which joins multiple `Matcher`s disjunctively.
///
/// Use `of()` to create a new `Matcher` and `or()` to add further `Matcher`s.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), Any::of(lt(0)).or(gt(5)).or(not(eq(3))));
/// # }
pub struct Any<T> {
    pub matcher: Box<Matcher<T>>,
    pub next: Option<Box<Any<T>>>
}

impl<T> Any<T> {
    /// Creates a new conjunctive `Matcher` starting with the given `Matcher`.
    pub fn of(matcher: Box<Matcher<T>>) -> Any<T> {
        Any {
            matcher: matcher,
            next: None
        }
    }

    /// Adds the given `Matcher` disjunctively.
    pub fn or(self, matcher: Box<Matcher<T>>) -> Any<T> {
        Any {
            matcher: matcher,
            next: Some(Box::new(self))
        }
    }
}

impl<T> Matcher<T> for Any<T> {
    fn check(&self, actual: &T) -> MatchResult {
        match self.matcher.check(actual) {
            MatchResult::Matched {..} => MatchResult::Matched { name: "any_of".to_owned() },
            x@MatchResult::Failed {..} => match self.next {
                None => x,
                Some(ref next) => next.check(actual)
            }
        }
    }
}
