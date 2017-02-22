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
