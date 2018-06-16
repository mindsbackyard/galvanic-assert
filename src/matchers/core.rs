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
        if $actual $comparison &$expected {
            builder.matched()
        } else {
            builder.failed_comparison($actual, &$expected)
        }
    }}
}

/// A matcher which always matches.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), assertion_always_succeeds());
/// # }
pub fn assertion_always_succeeds<'a,T:'a>() -> Box<Matcher<T> + 'a> {
    Box::new(|_s: &T| MatchResultBuilder::for_("succeeds_always").matched())
}
/// A matcher which always matches.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), any_value());
/// # }
pub fn any_value<'a,T:'a>() -> Box<Matcher<T> + 'a> {
    Box::new(|_s: &T| MatchResultBuilder::for_("any_value").matched())
}

/// A matcher which never matches.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(
///     assert_that!(&(1+1), assertion_always_fails()),
///     panics
/// );
/// # }
pub fn assertion_always_fails<'a,T:'a>() -> Box<Matcher<T> + 'a> {
    Box::new(|_s: &T| {
        MatchResultBuilder::for_("fails_always").failed_because("This matcher fails always")
    })
}
/// A matcher which never matches.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(
///     assert_that!(&(1+1), no_value()),
///     panics
/// );
/// # }
pub fn no_value<'a,T:'a>() -> Box<Matcher<T> + 'a> { assertion_always_fails() }

/// Accepts a matcher and returns it unmodified.
///
/// This is just syntactic sugar.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), is(eq(2)));
/// # }
pub fn is<'a, T:'a>(matcher: Box<Matcher<T> + 'a>) -> Box<Matcher<T> + 'a> {
    matcher
}

/// Accepts a matcher and returns it unmodified.
///
/// This is just syntactic sugar.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// let vs = vec![1, 2];
/// assert_that!(&vs.len(), has(lt(3)));
/// # }
pub fn has<'a, T:'a>(matcher: Box<Matcher<T> + 'a>) -> Box<Matcher<T> + 'a> {
    matcher
}

/// A matcher negating the result of the passed matcher.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), not(eq(3)));
/// # }
pub fn not<'a, T: 'a>(matcher: Box<Matcher<T> + 'a>) -> Box<Matcher<T> + 'a> {
    Box::new(move |actual: &T| {
        match matcher.check(actual) {
            MatchResult::Matched { name } =>
                MatchResultBuilder::for_(&format!("not({})", name))
                                   .failed_because(&format!("{} is satisfied", name)),
            MatchResult::Failed { name, .. } =>
                MatchResultBuilder::for_(&format!("not({})", name)).matched()
        }
    })
}

/// Matches if the asserted value is equal to the expected value.
///
/// This matcher should not be used when asserting floating point values.
/// Use [close_to] instead. This is the same as [eq].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), equal_to(2));
/// # }
pub fn equal_to<'a, T>(expected: T) -> Box<Matcher<T> + 'a>
where T: PartialEq + Debug + 'a {
    Box::new(move |actual: &T| matchresult_from_comparison!(actual == expected, "equal"))
}
/// Matches if the asserted value is equal to the expected value.
///
/// This matcher should not be used when asserting floating point values.
/// Use [close_to] instead. This is the same as [equal_to].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), eq(2));
/// # }
pub fn eq<'a, T: PartialEq + Debug + 'a>(expected: T) -> Box<Matcher<T> + 'a> { equal_to(expected) }

/// Matches if the asserted value is less than the expected value.
///
/// This is the same as [lt].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), less_than(3));
/// # }
pub fn less_than<'a, T>(expected: T) -> Box<Matcher<T> + 'a>
where T: PartialOrd + Debug + 'a {
    Box::new(move |actual: &T| matchresult_from_comparison!(actual < expected, "less_than"))
}
/// Matches if the asserted value is less than the expected value.
///
/// This is the same as [less_than].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), less_than(3));
/// # }
pub fn lt<'a, T: PartialOrd + Debug + 'a>(expected: T) -> Box<Matcher<T> + 'a> { less_than(expected) }

/// Matches if the asserted value is greater than the expected value.
///
/// This is the same as [gt].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), greater_than(1));
/// # }
pub fn greater_than<'a, T>(expected: T) -> Box<Matcher<T> + 'a>
where T: PartialOrd + Debug + 'a {
    Box::new(move |actual: &T| matchresult_from_comparison!(actual > expected, "greater_than"))
}
/// Matches if the asserted value is greater than the expected value.
///
/// This is the same as [greater_than].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), gt(1));
/// # }
pub fn gt<'a, T: PartialOrd + Debug + 'a>(expected: T) -> Box<Matcher<T> + 'a> { greater_than(expected) }

/// Matches if the asserted value is less than or equal to the expected value.
///
/// This is the same as [leq].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), less_than_or_equal(3));
/// assert_that!(&(1+1), less_than_or_equal(2));
/// # }
pub fn less_than_or_equal<'a, T>(expected: T) -> Box<Matcher<T> + 'a>
where T: PartialOrd + Debug + 'a {
    Box::new(move |actual: &T| matchresult_from_comparison!(actual <= expected, "less_than_or_equal"))
}
/// Matches if the asserted value is less than or equal to the expected value.
///
/// This is the same as [less_than_or_equal].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), leq(3));
/// assert_that!(&(1+1), leq(2));
/// # }
pub fn leq<'a, T: PartialOrd + Debug + 'a>(expected: T) -> Box<Matcher<T> + 'a> { less_than_or_equal(expected) }

/// Matches if the asserted value is greater than or equal to the expected value.
///
/// This is the same as [geq].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), greater_than_or_equal(1));
/// assert_that!(&(1+1), greater_than_or_equal(2));
/// # }
pub fn greater_than_or_equal<'a, T>(expected: T) -> Box<Matcher<T> + 'a>
where T: PartialOrd + Debug + 'a {
    Box::new(move |actual: &T| matchresult_from_comparison!(actual >= expected, "greater_than_or_equal"))
}
/// Matches if the asserted value is greater than or equal to the expected value.
///
/// This is the same as [greater_than_or_equal].
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1+1), geq(1));
/// assert_that!(&(1+1), geq(2));
/// # }
pub fn geq<'a, T: PartialOrd + Debug + 'a>(expected: T) -> Box<Matcher<T> + 'a> { greater_than_or_equal(expected) }

/// Matches if the asserted value is in an epsilon range around the expected value.
///
/// If floating point values are compared for equality this matcher should be used instead of [equal_to]
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// assert_that!(&(1.2 + 3.14), close_to(4.34, 0.00001));
/// # }
pub fn close_to<'a, T>(expected: T, eps: T) -> Box<Matcher<T> + 'a>
where T: Copy + PartialOrd + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Debug + 'a {
    Box::new(move |actual: &T| {
        let builder = MatchResultBuilder::for_("close_to");
        if &(expected - eps) <= actual && actual <= &(expected + eps) {
            builder.matched()
        } else {
            builder.failed_because(&format!("{:?} should be between {:?} and {:?}",
                                            actual, expected - eps, expected + eps)
            )
        }
    })
}

/// Matches if asserted value and the expected value are truely the same object.
///
/// The two values are the same if the reside at the same memory address.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// #[derive(Debug)]
/// struct Foo;
/// let foo1 = Foo;
/// let foo2 = Foo;
///
/// assert_that!(&foo1, same_object(&foo1));
///
/// assert_that!(
///     assert_that!(&foo1, same_object(&foo2)),
///     panics
/// );
/// # }
pub fn same_object<'a, T>(expected: &'a T) -> Box<Matcher<T> + 'a>
where T: Debug + 'a {
    Box::new(move |actual: &T| {
        let builder = MatchResultBuilder::for_("same_object");
        if (actual as *const _) == (expected as *const _) {
            builder.matched()
        } else {
            builder.failed_comparison(&actual, &expected)
        }
    })
}

/// Write patterns of structs/enums which use `Matcher`s instead of field values.
///
/// When providing matchers for multiple fields, not that *all* matchers will be evaluated.
/// Even if one of them returns `MatchResult::Failed`.
///
/// For struct-like structs/enum-variants not all fields need to be listed in the pattern.
/// Unwanted fields can safely be ommitted. For tuple-like structs/enum-variants all fields
/// need to be listed in the correct order. Although you can use `any_value()`
/// to effectively ignore the field.
///
/// Note that the correct brace/bracket style for tuple-like structs/enums is `Variant[any_value(), any_value()]`
/// not `Variant(any_value(), any_value())`. This discrepancy is due to macro parsing reasons.
///
/// #Examples
/// ```rust
/// // Matching a struct-like ...
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// # fn main() {
/// struct Foo { x: i32, y: f64 }
/// let foo = Foo { x: 12, y: 23.4 };
/// assert_that!(&foo, has_structure!(Foo {
///     x: eq(12),
///     y: lt(25.0)
/// }));
/// assert_that!(&foo, has_structure!(Foo {
///     y: lt(25.0) // not all fields need to be given for struct-likes
/// }));
///
/// // Matching a tuple-like ...
/// struct Bar(i32, f64);
/// let bar = Bar(12, 23.4);
/// assert_that!(&bar, has_structure!(Bar[ eq(12), lt(25.0) ]));
///
/// // Matching enum variants ...
/// enum Baz {
///     Var1 { x: i32, y: f64 },
///     Var2(i32, f64)
/// }
/// let var1 = Baz::Var1 { x: 12, y: 23.4 };
/// assert_that!(&var1, has_structure!(Baz::Var1 {
///     x: eq(12),
///     y: lt(25.0)
/// }));
///
/// let var2 = Baz::Var2(12, 23.4);
/// assert_that!(&var2, has_structure!(Baz::Var2 [eq(12), lt(25.0)] ));
/// # }
#[macro_export]
macro_rules! has_structure {
    ( $variant:path { $( $field:ident : $matcher:expr ),* $(,)* } ) => { structure!($variant { $($field : $matcher),* }) };
    ( $variant:path [ $( $matchers:expr ),* ] ) => { structure!($variant [ $($matchers),* ]) }
}
/// Shorter name for `has_structure!`.
#[macro_export]
macro_rules! structure {
    ( $variant:path { $( $field:ident : $matcher:expr ),* $(,)* } ) => {
        Box::new(|actual: &_| {
            use galvanic_assert::{MatchResultBuilder, MatchResult};
            let builder = MatchResultBuilder::for_("has_structure");
            #[allow(unreachable_patterns)]
            match actual {
                &$variant { $( ref $field, )* ..} => {
                    let mut failed_msgs = Vec::new();
                    $(
                        if let MatchResult::Failed{ name, reason } = $matcher.check($field) {
                            failed_msgs.push(
                                format!("Matcher '{}' for field '{}' at {}:{} failed:\n\t{}",
                                        name, stringify!($field), file!().to_string(), line!(), reason)
                            );
                        }
                    )*
                    if failed_msgs.is_empty() { builder.matched() }
                    else { builder.failed_because(&failed_msgs.join("\n")) }
                },
                _ => builder.failed_because(
                        &format!("passed variant does not match '{}'", stringify!($variant))
                )
            }
        })
    };

    (@expand ( $variant:path ; $field:ident ; $m:expr ; $($wildcard:tt),* ) -> ($($body:tt)*) ) => {
        structure!(@generate ($field ; $($body)* ($m ; &$variant($($wildcard,)* ref $field))) )
    };
    (@expand ( $variant:path ; $field:ident ; $m:expr , $($matchers:expr),* ; $($wildcard:tt),* ) -> ($($body:tt)*) ) => {
        structure!(@expand ( $variant ; $field ; $($matchers),* ; $($wildcard,)* _ ) -> ($($body)* ($m ; &$variant($($wildcard,)* ref $field, ..)),) )
    };
    (@generate ($field:ident ; $(($matcher:expr ; $pattern:pat)),*) ) => {
        Box::new(|actual: &_| {
            use galvanic_assert::{MatchResultBuilder, MatchResult};
            let builder = MatchResultBuilder::for_("has_structure");

            let mut failed_msgs = Vec::new();
            $(
                #[allow(unreachable_patterns)]
                match actual {
                    $pattern => if let MatchResult::Failed{ name, reason } = $matcher.check($field) {
                        failed_msgs.push(
                            format!("Matcher '{}' for field '{}' at {}:{} failed:\n\t{}",
                                    name, stringify!($field), file!().to_string(), line!(), reason)
                        );
                    },
                    _ => return builder.failed_because(
                            &format!("passed variant does not match '{}'", stringify!($variant))
                    )
                }
            )*

            if failed_msgs.is_empty() { builder.matched() }
            else { builder.failed_because(&failed_msgs.join("\n")) }
        })
    };
    ( $variant:path [ $( $matchers:expr ),* ] ) => {
        structure![ @expand ( $variant ; x ; $($matchers),* ; ) -> () ]
    };
}
