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
/// ```rust,ignore
/// assert_that!(Ok(4), is_variant!(Ok));
/// ```rust,ignore
/// If not then the full path of the variant has to be used:
/// ```rust,ignore
/// enum MyEnum { Foo, Bar(i32), Baz{x: i32} }
///
/// assert_that!(MyEnum::Baz{x: 2}, is_variant!(MyEnum::Baz));
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
pub fn maybe_some<'a, T: 'a>(matcher: Box<Matcher<'a,T> + 'a>) -> Box<Matcher<'a,Option<T>> + 'a> {
    Box::new(move |maybe_actual: &'a Option<T>| {
        maybe_actual.as_ref()
                    .map_or(MatchResultBuilder::for_("maybe_some")
                                               .failed_because("passed Option is None; cannot evaluate nested matcher"),
                            |actual| matcher.check(actual)
        )
    })
}

/// Matches the contents of a `Result` if it is `Ok` againts a passed `Matcher`.
pub fn maybe_ok<'a, T: 'a, E: 'a>(matcher: Box<Matcher<'a,T> + 'a>) -> Box<Matcher<'a,Result<T,E>> + 'a> {
    Box::new(move |maybe_actual: &'a Result<T,E>| {
        match maybe_actual.as_ref() {
            Ok(actual) => matcher.check(actual),
            Err(_) => MatchResultBuilder::for_("maybe_ok")
                                       .failed_because("passed Result is Err; cannot evaluate nested matcher")
        }
    })
}

/// Matches the contents of a `Result` if it is `Err` againts a passed `Matcher`.
pub fn maybe_err<'a, T: 'a, E: 'a>(matcher: Box<Matcher<'a,E> + 'a>) -> Box<Matcher<'a,Result<T,E>> + 'a> {
    Box::new(move |maybe_actual: &'a Result<T,E>| {
        match maybe_actual.as_ref() {
            Err(actual) => matcher.check(actual),
            Ok(_) => MatchResultBuilder::for_("maybe_err")
                                        .failed_because("passed Result is Ok; cannot evaluate nested matcher")
        }
    })
}
