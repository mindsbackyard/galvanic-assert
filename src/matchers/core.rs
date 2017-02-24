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

//! The core module contains the basic matchers needed for writing assertions.
//!
//! The matchers in this module all operate on single values.

use std::fmt::Debug;
use super::super::*;

macro_rules! matchresult_from_comparison {
    (  $actual: ident $comparison: tt $expected: ident, $name: expr ) => {{
        let builder = MatchResultBuilder::for_($name);
        if $actual $comparison $expected {
            builder.matched()
        } else {
            builder.failed_comparison(&$actual, &$expected)
        }
    }}
}

/// A matcher which always matches.
pub fn assertion_always_succeeds<T>() -> impl Fn(T) -> MatchResult {
    move |_s: T| MatchResultBuilder::for_("succeeds_always").matched()
}

/// A matcher which never matches.
pub fn assertion_always_fails<T>() -> impl Fn(T) -> MatchResult {
    move |_s: T| {
        MatchResultBuilder::for_("fails_always").failed_because("This matcher fails always")
    }
}

/// Accepts a matcher and returns it unmodified.
///
/// This is just syntactic sugar.
pub fn is<T, M>(matcher: M) -> M
where M: Matcher<T> {
    matcher
}

/// A matcher negating the result of the passed matcher.
pub fn not<T, M>(matcher: M) -> impl Fn(T) -> MatchResult
where M: Matcher<T>, {
    move |actual: T| {
        match matcher.check(actual) {
            MatchResult::Matched { name } =>
                MatchResultBuilder::for_(&format!("not({})", name))
                                   .failed_because(&format!("{} is satisfied", name)),
            MatchResult::Failed { name, .. } =>
                MatchResultBuilder::for_(&format!("not({})", name)).matched()
        }
    }
}

/// Matches if the asserted value is equal to the expected value.
///
/// This matcher should not be used when asserting floating point values.
/// Use [close_to] instead.
pub fn equal_to<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialEq + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual == expected, "equal")
}
/// Matches if the asserted value is equal to the expected value.
pub fn eq<T: PartialEq + Debug>(expected: T) -> impl Fn(T) -> MatchResult { equal_to(expected) }

/// Matches if the asserted value is less than the expected value.
pub fn less_than<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual < expected, "less_than")
}
/// Matches if the asserted value is less than the expected value.
pub fn lt<T: PartialOrd + Debug>(expected: T) -> impl Fn(T) -> MatchResult { less_than(expected) }

/// Matches if the asserted value is greater than the expected value.
pub fn greater_than<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual > expected, "greater_than")
}
/// Matches if the asserted value is greater than the expected value.
pub fn gt<T: PartialOrd + Debug>(expected: T) -> impl Fn(T) -> MatchResult { greater_than(expected) }

/// Matches if the asserted value is less than or equal to the expected value.
pub fn less_than_or_equal<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual <= expected, "less_than_or_equal")
}
/// Matches if the asserted value is less than or equal to the expected value.
pub fn leq<T: PartialOrd + Debug>(expected: T) -> impl Fn(T) -> MatchResult { less_than_or_equal(expected) }

/// Matches if the asserted value is greater than or equal to the expected value.
pub fn greater_than_or_equal<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual >= expected, "greater_than_or_equal")
}
/// Matches if the asserted value is greater than or equal to the expected value.
pub fn geq<T: PartialOrd + Debug>(expected: T) -> impl Fn(T) -> MatchResult { greater_than_or_equal(expected) }

/// Matches if the asserted value is in an epsilon range around the expected value.
///
/// If floating point values are compared for equality this matcher should be used instead of [equal_to]
pub fn close_to<T>(expected: T, eps: T) -> impl Fn(T) -> MatchResult
where T: Copy + PartialOrd + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Debug {
    move |actual: T| {
        let builder = MatchResultBuilder::for_("close_to");
        if expected - eps <= actual && actual <= expected + eps {
            builder.matched()
        } else {
            builder.failed_because(&format!("{:?} should be between {:?} and {:?}",
                                            actual, expected - eps, expected + eps)
            )
        }
    }
}

/// Matches if asserted value and the expected value are truely the same object.
///
/// The two values are the same if the reside at the same memory address.
pub fn same_object<'a, T>(expected: &'a T) -> impl Fn(&T) -> MatchResult
where T: Debug {
    move |actual: &T| {
        let builder = MatchResultBuilder::for_("same_object");
        if (actual as *const _) == (expected as *const _) {
            builder.matched()
        } else {
            builder.failed_comparison(&actual, &expected)
        }
    }
}
