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

//! Galvanic-assert: Matcher-based assertions for easier testing
//! ============================================================
//! This crate provides a new assertion macro `assert_that!` based on **matching predicates** (matchers) to
//!
//!  * make **writing** asserts easier
//!  * make **reading** asserts comprehendable
//!  * easily **extend** the assertion framework
//!  * provide a large list **common matchers**
//!  * integrate with **galvanic-test** and **galvanic-mock** (both still in development ... stay tuned!)
//!  * be used with your favourite test framework
//!
//! The crate will be part of **galvanic**---a complete test framework for **Rust**.
//!
//! **galvanic-assert** currently requires *nightly* until *impl trait* returns have been stabilized.

#![feature(conservative_impl_trait)]
#![feature(discriminant_value)]

use std::fmt::{Display, Debug, Formatter, Result as FormatResult};

/// States that the asserted values satisfies the required properties of the supplied `Matcher`.
///
/// The macro comes in three different forms:
///
///  1. Assert that some expression is true, supplied with an optional error message.
///
///     ```rust,ignore
///     assert_that!(EXPRESSION);
///     assert_that!(EXPRESSION, otherwise "some error message");
///     ```
///  2. Assert that some expression satifies the properties of some `Matcher`.
///     The `Matcher` is either predefined, a user defined type with a `Matcher` implementation, or a closure returning a `MatchResult`
///
///     ```rust,ignore
///     assert_that!(1, eq(1));
///     assert_that!(1, |x| {
///         let builder = MatchResultBuilder::for_("my_matcher");
///         if x == 1 { builder.matched } else { builder.failed_because("some reason") }
///     })
///     ```
///  3. Assert that some expression is expected to panic/not panic.
///
///     ```rust,ignore
///     assert_that!(panic!("panic"), panics);
///     assert_that!(1+1, does not panic);
///     ```
#[macro_export]
macro_rules! assert_that {
    ( $actual: expr, panics ) => {
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_ok() {
            panic!("\nFailed assertion; expected expression to panic")
        }
    };
    ( $actual: expr, does not panic ) => {
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_err() {
            panic!("\nFailed assertion; expression panicked unexpectantly")
        }
    };
    ( $actual: expr) => {
        if !$actual {
            panic!("\nFailed assertion; '{}' is not true", stringify!($actual));
        }
    };
    ( $actual: expr , otherwise $reason: expr ) => {
        if !$actual {
            panic!("\nFailed assertion; expression '{}' is not true,\n    Because: {}",
                   stringify!($actual), $reason
            );
        }
    };
    ( $actual: expr, $matcher: expr ) => {
        match $matcher.check($actual) {
            MatchResult::Matched { .. } => { },
            MatchResult::Failed { name, reason } => {
                panic!("\nFailed assertion of matcher: {}\n{}", name, reason)
            }
        }
    };
}

/// The trait which has to be implemented by all matchers.
pub trait Matcher<T> {
    /// Checks the passed value if it satisfies the `Matcher`.
    fn check(&self, actual: T) -> MatchResult;
}

/// A closures can be used as a `Matcher`.
///
/// The closure must be repeatably callable in case that the matcher is combined with another matcher.
impl<T, F> Matcher<T> for F
where F: Fn(T) -> MatchResult {
    fn check(&self, actual: T) -> MatchResult {
        self(actual)
    }
}

/// The return type of any `Machter`
pub enum MatchResult {
    /// Indicates that the `Matcher` matched the value under inspection.
    Matched {
        /// The `name` of the `Matcher`
        name: String
    },
    /// Indicates that the `Matcher` failed to match the value under inspection.
    Failed {
        /// The `name` of the `Matcher`
        name: String,
        /// The `reason` why the `Matcher` failed
        reason: String
    }
}

/// A builder for creating `MatchResult`s.
///
/// Create a new builder with `new()` or `for_()`
/// and finalize it either with `matched()`, `failed_because()`, or `failed_comparison()`.
pub struct MatchResultBuilder {
    matcher_name: String
}

impl MatchResultBuilder {
    /// Creates a `MatchResultBuilder` for an anonymous `Matcher`.
    pub fn new() -> MatchResultBuilder {
        MatchResultBuilder {
            matcher_name: "_unknown_".to_owned()
        }
    }

    /// Creates `MatchResultBuilder` for a `Matcher` with the given `name`
    pub fn for_(name: &str) -> MatchResultBuilder {
        MatchResultBuilder {
            matcher_name: name.to_owned()
        }
    }

    /// Finalzes the builder indicating that the `Matcher` matched the inspected value.
    pub fn matched(self) -> MatchResult {
        MatchResult::Matched { name: self.matcher_name }
    }

    /// Finalzes the builder indicating that the `Matcher` failed to the inspected value.
    ///
    /// The `reason` should give a short indication why the matcher failed.
    pub fn failed_because(self, reason: &str) -> MatchResult {
        MatchResult::Failed {
            name: self.matcher_name,
            reason: format!("  Because: {}", reason)
        }
    }

    /// Finalzes the builder indicating that the `Matcher` failed to the inspected value.
    ///
    /// The `actual` and `expected` value are used the generate a useful error message.
    pub fn failed_comparison<T: Debug>(self, actual: &T, expected: &T) -> MatchResult {
        MatchResult::Failed {
            name: self.matcher_name,
            reason: format!("  Expected: {:?}\n  Got: {:?}", expected, actual)
        }
    }
}

pub mod matchers;
