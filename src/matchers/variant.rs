use std::fmt::Debug;
use super::super::*;

use std::mem;

#[macro_export]
macro_rules! is_variant {
    ( $enum_type: path, $variant: path ) => {
        |actual: &_| {
            match *actual {
                $variant {..} => MatchResult::Matched { name: "is_variant".to_owned() },
                _ => MatchResult::Failed {
                        name: "is_variant".to_owned(),
                        reason: format_fail_reason(&format!("passed variant does not match '{}'", stringify!($variant)))
                }
            }
        }
    }
}

pub fn same_variant_as<'a, T>(expected: &'a T) -> impl Fn(&T) -> MatchResult
where T: Debug {
    move |actual: &T| {
        if mem::discriminant(actual) == mem::discriminant(expected) {
            MatchResult::Matched { name: "same_variant_as".to_owned() }

        } else {
            MatchResult::Failed {
                    name: "same_variant_as".to_owned(),
                    reason: format_fail_comparison(actual, &expected)
            }
        }
    }
}
