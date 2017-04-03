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
//! This crate provides a new assertion macros (`assert_that!`, `expect_that!`, `get_expectation_for!`) based on **matching predicates** (matchers) to
//!
//!  * make **writing** asserts easier
//!  * make **reading** asserts comprehendable
//!  * easily **extend** the assertion framework
//!  * provide a large list **common matchers**
//!  * integrate with **galvanic-test** and **galvanic-mock** (both still in development ... stay tuned!)
//!  * be used with your favourite test framework
//!
//! The crate will be part of **galvanic**---a complete test framework for **Rust**.

use std::fmt::{Debug, Display, Formatter, Result as FormatResult};

/// States that the asserted values satisfies the required properties of the supplied `Matcher`.
///
/// The postulated assertion is verfied immediately and panics if it is not satisfied.
/// The macro comes in three different forms:
///
///  1. Assert that some expression is true, supplied with an optional error message.
///
///     ```rust,ignore
///     assert_that!(EXPRESSION);
///     assert_that!(EXPRESSION, otherwise "some error message");
///     ```
///  2. Assert that some expression satifies the properties of some `Matcher`.
///     The `Matcher` is either predefined, a user defined type with a `Matcher` implementation, or a closure returning a `MatchResult`.
///
///     ```rust,ignore
///     assert_that!(&1, eq(1));
///     assert_that!(&1, |x| {
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
    ( $actual: expr, panics ) => {{
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_ok() {
            panic!("\nFailed assertion; expected expression to panic")
        }
    }};
    ( $actual: expr, does not panic ) => {
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_err() {
            panic!("\nFailed assertion; expression panicked unexpectantly")
        }
    };
    ( $actual: expr) => {{
        if !$actual {
            panic!("\nFailed assertion; '{}' is not true", stringify!($actual));
        }
    }};
    ( $actual: expr , otherwise $reason: expr ) => {{
        if !$actual {
            panic!("\nFailed assertion; expression '{}' is not true,\n    Because: {}",
                   stringify!($actual), $reason
            );
        }
    }};
    ( $actual: expr, $matcher: expr ) => {{
        #[allow(unused_imports)]
        use galvanic_assert::{MatchResult, Matcher};
        //use std::borrow::Borrow;
        // store the actual value to borrow it
        let value = $actual;
        // use borrow to obtain a reference of type &T (either from a value of type T or of &T itself---the beauty of the Borrow)
        //let ref_to_value: &_ = value.borrow();
        // store matcher so it's dropped before the actual value (reverse order of declaration)
        let m = $matcher;
        match m.check(value) {
            MatchResult::Matched { .. } => { },
            MatchResult::Failed { name, reason } => {
                panic!("\nFailed assertion of matcher: {}\n{}", name, reason)
            }
        }
    }};
}

/// States that the asserted values satisfies the required properties of the supplied `Matcher`
/// and returns an `Expectation` object to inspect the results at a later time.
///
/// The postulated assertion is verfied immediately,
/// but the returned `Expectation` defers a potential panic either until `Expectation::verify` is called
/// or the `Expectation` object is dropped.
/// It is safe for multiple expectations to fail the assertion code will prevent nested panics.
///
/// The macro comes in three different forms:
///
///  1. Expect that some expression is true, supplied with an optional error message.
///
///     ```rust,ignore
///     let e1 = get_expectation_for!(EXPRESSION);
///     let e2 = get_expectation_for!(EXPRESSION, otherwise "some error message");
///     ```
///  2. Expect that some expression satifies the properties of some `Matcher`.
///     The `Matcher` is either predefined, a user defined type with a `Matcher` implementation, or a closure returning a `MatchResult`.
///
///     ```rust,ignore
///     let e1 = get_expectation_for!(&1, eq(1));
///     let e2 = get_expectation_for!(&1, |x| {
///         let builder = MatchResultBuilder::for_("my_matcher");
///         if x == 1 { builder.matched } else { builder.failed_because("some reason") }
///     })
///     ```
///  3. Expect that some expression is expected to panic/not panic.
///
///     ```rust,ignore
///     let e1 = get_expectation_for!(panic!("panic"), panics);
///     let e2 = get_expectation_for!(&1+1, does not panic);
///     ```
///
/// An expectation can be verfied manually
///
/// ```rust,ignore
/// let e1 = get_expectation_for!(&1+1, equal_to(0));
/// let e2 = get_expectation_for!(&1+1, less_than(4)); // is executed
/// e1.verify();
/// let e3 = get_expectation_for!(&1+1, panics); // is never executed as e1 panics
/// ```
/// or is automatically verfied on drop.
///
/// ```rust,ignore
/// {
///     let e1 = get_expectation_for!(&1+1, equal_to(0));
///     let e2 = get_expectation_for!(&1+1, less_than(4)); // is executed
/// }
/// let e3 = get_expectation_for!(1+1, panics); // is never executed as e1 panics
/// ```
#[macro_export]
macro_rules! get_expectation_for {
    ( $actual: expr, panics ) => {{
        use galvanic_assert::Expectation;
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_ok() {
            let assertion = format!("'{}, panics'", stringify!($actual));
            Expectation::failed(assertion, file!().to_string(), line!(),
                                "Expected expression to panic".to_string()
            )
        } else {
            Expectation::satisfied()
        }
    }};
    ( $actual: expr, does not panic ) => {{
        use galvanic_assert::Expectation;
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_err() {
            let assertion = format!("'{}, does not panic'", stringify!($actual));
            Expectation::failed(assertion, file!().to_string(), line!(),
                                "Expression panicked unexpectantly".to_string()
            )
        } else { Expectation::satisfied() }
    }};
    ( $actual: expr) => {{
        use galvanic_assert::Expectation;
        if !$actual {
            let assertion = format!("'{}' is true", stringify!($actual));
            Expectation::failed(assertion, file!().to_string(), line!(),
                                format!("'{}' is not true", stringify!($actual))
            )
        } else { Expectation::satisfied() }
    }};
    ( $actual: expr , otherwise $reason: expr ) => {{
        use galvanic_assert::Expectation;
        if !$actual {
            let assertion = format!("'{}' is true", stringify!($actual));
            Expectation::failed(assertion, file!().to_string(), line!(),
                                format!("'{}' is not true,\n\tBecause: {}",
                                        stringify!($actual), $reason)
            )
        } else { Expectation::satisfied() }
    }};
    ( $actual: expr, $matcher: expr ) => {{
        #[allow(unused_imports)]
        use galvanic_assert::{Expectation, MatchResult, Matcher};
        let value = $actual;
        let m = $matcher;
        match m.check(value) {
            MatchResult::Matched { .. } => { Expectation::satisfied() },
            MatchResult::Failed { name, reason } => {
                let assertion = format!("'{}' matches '{}'", stringify!($actual), stringify!($matcher));
                Expectation::failed(assertion, file!().to_string(), line!(),
                                    format!("Failed assertion of matcher: {}\n{}", name, reason)
                )
            }
        }
    }};
}

/// States that the asserted values satisfies the required properties of the supplied `Matcher`
/// but waits until the end of the block to inspect the results.
///
/// The postulated assertion is verfied immediately,
/// but a potential panic is deferred until the end of the block wherein the expectation is stated.
/// It is safe for multiple expectations to fail.
/// The assertion code will prevent nested panics.
///
/// The macro comes in three different forms:
///
///  1. Expect that some expression is true, supplied with an optional error message.
///
///     ```rust,ignore
///     expect_that!(EXPRESSION);
///     expect_that!(EXPRESSION, otherwise "some error message");
///     ```
///  2. Expect that some expression satifies the properties of some `Matcher`.
///     The `Matcher` is either predefined, a user defined type with a `Matcher` implementation, or a closure returning a `MatchResult
///
///     ```rust,ignore
///     expect_that!(&1, eq(1));
///     expect_that!(&1, |x| {
///         let builder = MatchResultBuilder::for_("my_matcher");
///         if x == 1 { builder.matched } else { builder.failed_because("some reason") }
///     })
///     ```
///  3. Expect that some expression is expected to panic/not panic.
///
///     ```rust,ignore
///     expect_that!(panic!("panic"), panics);
///     expect_that!(1+1, does not panic);
///     ```
///
/// An expectation is verified at the end of the block it is stated in:
///
/// ```rust,ignore
/// {
///     expect_that!(&1+1, equal_to(0));
///     expect_that!(&1+1, less_than(4)); // is executed
/// }
/// expect_that!(1+1, panics); // is never executed as e1 panics
/// ```
#[macro_export]
macro_rules! expect_that {
    ( $actual: expr, panics ) => { #[allow(unused_variables)] let expectation = get_expectation_for!($actual, panics); };
    ( $actual: expr, does not panic ) => { #[allow(unused_variables)] let expectation = get_expectation_for!($actual, does not panic); };
    ( $actual: expr) => { #[allow(unused_variables)] let expectation = get_expectation_for!($actual); };
    ( $actual: expr , otherwise $reason: expr ) => { #[allow(unused_variables)] let expectation = get_expectation_for!($actual, otherwise $reason); };
    ( $actual: expr, $matcher: expr ) => { #[allow(unused_variables)] let expectation = get_expectation_for!($actual, $matcher); };
}

/// The trait which has to be implemented by all matchers.
pub trait Matcher<'a, T:'a> {
    /// Checks the passed value if it satisfies the `Matcher`.
    ///
    /// Values are always taken as immutable reference as the actual value shouldn't be changed by the matcher.
    fn check(&self, actual: &'a T) -> MatchResult;
}

/// A closures can be used as a `Matcher`.
///
/// The closure must be repeatably callable in case that the matcher is combined with another matcher.
impl<'a, T:'a, F> Matcher<'a,T> for F
where F: Fn(&'a T) -> MatchResult + ?Sized {
    fn check(&self, actual: &'a T) -> MatchResult {
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

/// The result of a deferred assertion.
///
///
pub enum Expectation {
    Failed {
        /// A representation of the failed assertion.
        assertion: String,
        /// The file where the expection has been created.
        file: String,
        /// The source code line where the expectation has been created.
        line: u32,
        /// The reason why the expectation has not been met.
        error_msg: String
    },
    Satisfied
}

impl Expectation {
    /// Creates a failed `Expectation`
    pub fn failed(assertion:String, file: String, line: u32, error_msg: String) -> Expectation {
        Expectation::Failed {
            assertion: assertion,
            file: file,
            line: line,
            error_msg: error_msg
        }
    }

    /// Create a satisfied `Expectation`
    pub fn satisfied() -> Expectation {
        Expectation::Satisfied
    }

    /// Verifies if the asseration given by the `Expectation` held.
    ///
    /// Panics if the verification fails.
    pub fn verify(self) { /* drop self */ }
}

/// If the `Expectation` is dropped it is automatically verified.
impl Drop for Expectation {
    fn drop(&mut self) {
        if let &mut Expectation::Failed { .. } = self {
            println!("{}", self);
            if !std::thread::panicking() {
                panic!("Some expectations failed.")
            }
        }
    }
}

impl Display for Expectation {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        match self {
            &Expectation::Failed { ref assertion, ref file, ref line, ref error_msg } => {
                write!(f, "Expectation '{}' failed, originating from {}:{}\n\t{}",
                       assertion, file, line, error_msg
                )
            },
            _ => write!(f, "The expectation has been satisfied")
        }
    }
}

pub mod matchers;
