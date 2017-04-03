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

//! The variant module contains matchers for asserting properties of enums.

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
