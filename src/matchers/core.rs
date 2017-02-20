use std::fmt::Debug;
use super::super::*;

macro_rules! matchresult_from_comparison {
    (  $actual: ident $comparison: tt $expected: ident, $name: expr ) => {
        if $actual $comparison $expected {
            MatchResult::Matched { name: $name.to_owned() }
        } else {
            MatchResult::Failed {
                name: $name.to_owned(),
                reason: format_fail_comparison(&$actual, &$expected)
            }
        }
    }
}

pub fn assertion_always_succeeds<T>() -> impl Fn(T) -> MatchResult {
    move |_s: T| MatchResult::Matched { name: "succeeds_always".to_owned() }
}

pub fn assertion_always_fails<T>() -> impl Fn(T) -> MatchResult {
    move |_s: T| MatchResult::Failed {
        name: "fails_always".to_owned(),
        reason: format_fail_reason("This matcher fails always")
    }
}

pub fn is<T, M>(matcher: M) -> M
where M: Matcher<T> {
    matcher
}

pub fn not<T, M>(matcher: M) -> impl Fn(T) -> MatchResult
where M: Matcher<T>, {
    move |actual: T| {
        match matcher.check(actual) {
            MatchResult::Matched { name } => MatchResult::Failed {
                name: format!("not({})", name),
                reason: format_fail_reason(&format!("{} is satisfied", name))
            },
            MatchResult::Failed { name, .. } => MatchResult::Matched { name: name },
        }
    }
}

pub fn eq<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialEq + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual == expected, "equal")
}

pub fn less_than<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual < expected, "less_than")
}
pub fn lt<T: PartialOrd + Debug>(expected: T) -> impl Fn(T) -> MatchResult { less_than(expected) }

pub fn greater_than<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual > expected, "greater_than")
}
pub fn gt<T: PartialOrd + Debug>(expected: T) -> impl Fn(T) -> MatchResult { greater_than(expected) }

pub fn less_than_or_equal<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual <= expected, "less_than_or_equal")
}
pub fn leq<T: PartialOrd + Debug>(expected: T) -> impl Fn(T) -> MatchResult { less_than_or_equal(expected) }

pub fn greater_than_or_equal<T>(expected: T) -> impl Fn(T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: T|
        matchresult_from_comparison!(actual >= expected, "greater_than_or_equal")
}
pub fn geq<T: PartialOrd + Debug>(expected: T) -> impl Fn(T) -> MatchResult { greater_than_or_equal(expected) }

pub fn close_to<T>(expected: T, eps: T) -> impl Fn(T) -> MatchResult
where T: Copy + PartialOrd + std::ops::Add<Output=T> + std::ops::Sub<Output=T> + Debug {
    move |actual: T|
        if expected - eps <= actual && actual <= expected + eps {
            MatchResult::Matched { name: "close_to".to_owned() }
        } else {
            MatchResult::Failed {
                name: "close_to".to_owned(),
                reason: format_fail_reason(
                    &format!("{:?} should be between {:?} and {:?}",
                        actual, expected - eps, expected + eps
                ))
            }
        }
}

pub fn same_object<'a, T>(expected: &'a T) -> impl Fn(&T) -> MatchResult
where T: Debug {
    move |actual: &T|
        if (actual as *const _) == (expected as *const _) {
            MatchResult::Matched { name: "same_object".to_owned() }
        } else {
            MatchResult::Failed {
                name: "same_object".to_owned(),
                reason: format_fail_comparison(&actual, &expected)
            }
        }
}
