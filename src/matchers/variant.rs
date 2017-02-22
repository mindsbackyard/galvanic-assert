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

use std::fmt::Debug;
use super::super::*;

use std::mem;

#[macro_export]
macro_rules! is_variant {
    ( $variant: path ) => {
        |actual| {
            let builder = MatchResultBuilder::for_("is_variant");
            match actual {
                $variant {..} => builder.matched(),
                _ => builder.failed_because(
                        &format!("passed variant does not match '{}'", stringify!($variant))
                )
            }
        }
    }
}

pub fn same_variant_as<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: Debug {
    move |actual: T| {
        let builder = MatchResultBuilder::for_("same_variant_as");
        if mem::discriminant(&actual) == mem::discriminant(&expected) {
            builder.matched()
        } else {
            builder.failed_comparison(&actual, &expected)
        }
    }
}
