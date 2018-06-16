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

//! The variant module contains matchers for asserting properties of enums and convienience functions for Option and Result.

use super::super::*;

/// Matches if the asserted value's variant matches the expected variant.
///
/// # Examples
/// If the enum's variants are already imported one can write:
///
/// ```
/// # #[macro_use] extern crate galvanic_assert;
/// # fn main() {
/// let ok: Result<i32, ()> = Ok(4);
/// assert_that!(&ok, is_variant!(Ok));
/// # }
/// ```
/// If not then the full path of the variant has to be used:
///
/// ```
/// # #[macro_use] extern crate galvanic_assert;
/// # fn main() {
/// enum MyEnum { Foo, Bar(i32), Baz{x: i32} }
/// assert_that!(&MyEnum::Baz{x: 2}, is_variant!(MyEnum::Baz));
/// # }
/// ```
#[macro_export]
macro_rules! is_variant {
    ( $variant: path ) => {
        Box::new(|actual: &_| {
            use galvanic_assert::MatchResultBuilder;
            let builder = MatchResultBuilder::for_("is_variant");
            match actual {
                &$variant {..} => builder.matched(),
                _ => builder.failed_because(
                        &format!("passed variant does not match '{}'", stringify!($variant))
                )
            }
        })
    }
}

/// Matches the contents of an `Option` againts a passed `Matcher`.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// use galvanic_assert::matchers::variant::*;
/// # fn main() {
/// assert_that!(&Some(32), maybe_some(eq(32)));
/// # }
pub fn maybe_some<'a, T:'a>(matcher: Box<Matcher<T> + 'a>) -> Box<Matcher<Option<T>> + 'a> {
    Box::new(move |maybe_actual: &Option<T>| {
        maybe_actual.as_ref()
                    .map_or(MatchResultBuilder::for_("maybe_some")
                                               .failed_because("passed Option is None; cannot evaluate nested matcher"),
                            |actual| matcher.check(actual)
        )
    })
}

/// Matches the contents of a `Result` if it is `Ok` againts a passed `Matcher`.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// use galvanic_assert::matchers::variant::*;
/// # fn main() {
/// let ok: Result<i32,()> = Ok(32);
/// assert_that!(&ok, maybe_ok(eq(32)));
/// # }
pub fn maybe_ok<'a, T:'a, E:'a>(matcher: Box<Matcher<T> + 'a>) -> Box<Matcher<Result<T,E>> + 'a> {
    Box::new(move |maybe_actual: &Result<T,E>| {
        match maybe_actual.as_ref() {
            Ok(actual) => matcher.check(actual),
            Err(_) => MatchResultBuilder::for_("maybe_ok")
                                       .failed_because("passed Result is Err; cannot evaluate nested matcher")
        }
    })
}

/// Matches the contents of a `Result` if it is `Err` againts a passed `Matcher`.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::*;
/// use galvanic_assert::matchers::variant::*;
/// # fn main() {
/// let err: Result<i32,i32> = Err(32);
/// assert_that!(&err, maybe_err(eq(32)));
/// # }
pub fn maybe_err<'a, T:'a, E:'a>(matcher: Box<Matcher<E> + 'a>) -> Box<Matcher<Result<T,E>> + 'a> {
    Box::new(move |maybe_actual: &Result<T,E>| {
        match maybe_actual.as_ref() {
            Err(actual) => matcher.check(actual),
            Ok(_) => MatchResultBuilder::for_("maybe_err")
                                        .failed_because("passed Result is Ok; cannot evaluate nested matcher")
        }
    })
}
