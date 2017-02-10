use std::fmt::Debug;
use super::super::*;

pub fn assertion_always_succeeds<T>() -> impl FnMut(&T) -> MatchResult {
    move |_s: &T| MatchResult::Matched { name: "succeeds_always".to_owned() }
}

pub fn assertion_always_fails<T>() -> impl FnMut(&T) -> MatchResult {
    move |_s: &T| MatchResult::Failed {
        name: "fails_always".to_owned(),
        reason: format_fail_reason("This matcher fails always")
    }
}

pub fn not<T, M>(mut matcher: M) -> impl FnMut(&T) -> MatchResult
where M: Matcher<T> {
    move |actual: &T| {
        match matcher.check(actual) {
            MatchResult::Matched { name } => MatchResult::Failed {
                name: format!("not({})", name),
                reason: format_fail_reason(&format!("{} is satisfied", name))
            },
            MatchResult::Failed { name, .. } => MatchResult::Matched { name: name },
        }
    }
}

pub fn eq<T>(expected: T) -> impl Fn(&T) -> MatchResult
where T: PartialEq + Debug {
    move |actual: &T|
        if *actual == expected {
            MatchResult::Matched { name: "equal".to_owned() }
        } else {
            MatchResult::Failed {
                name: "eq".to_owned(),
                reason: format_fail_comparison(actual, &expected)
            }
        }
}

pub fn less_than<T>(expected: T) -> impl Fn(&T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: &T|
        if *actual < expected {
            MatchResult::Matched { name: "less_than".to_owned() }
        } else {
            MatchResult::Failed {
                name: "less_than".to_owned(),
                reason: format_fail_comparison(actual, &expected)
            }
        }
}
pub fn lt<T: PartialOrd + Debug>(expected: T) -> impl Fn(&T) -> MatchResult { less_than(expected) }

pub fn greater_than<T>(expected: T) -> impl Fn(&T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: &T|
        if *actual > expected {
            MatchResult::Matched { name: "greater_than".to_owned() }
        } else {
            MatchResult::Failed {
                name: "greater_than".to_owned(),
                reason: format_fail_comparison(actual, &expected)
            }
        }
}
pub fn gt<T: PartialOrd + Debug>(expected: T) -> impl Fn(&T) -> MatchResult { greater_than(expected) }

pub fn less_than_or_equal<T>(expected: T) -> impl Fn(&T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: &T|
        if *actual <= expected {
            MatchResult::Matched { name: "less_than_or_equal".to_owned() }
        } else {
            MatchResult::Failed {
                name: "less_than_or_equal".to_owned(),
                reason: format_fail_comparison(actual, &expected)
            }
        }
}
pub fn leq<T: PartialOrd + Debug>(expected: T) -> impl Fn(&T) -> MatchResult { less_than_or_equal(expected) }

pub fn greater_than_or_equal<T>(expected: T) -> impl Fn(&T) -> MatchResult
where T: PartialOrd + Debug {
    move |actual: &T|
        if *actual >= expected {
            MatchResult::Matched { name: "greater_than_or_equal".to_owned() }
        } else {
            MatchResult::Failed {
                name: "greater_than_or_equal".to_owned(),
                reason: format_fail_comparison(actual, &expected)
            }
        }
}
pub fn geq<T: PartialOrd + Debug>(expected: T) -> impl Fn(&T) -> MatchResult { greater_than_or_equal(expected) }

pub fn is_same_object<'a, T>(expected: &'a T) -> impl Fn(&T) -> MatchResult
where T: PartialEq + Debug {
    move |actual: &T|
        if actual == expected {
            MatchResult::Matched { name: "is_same_object".to_owned() }
        } else {
            MatchResult::Failed {
                name: "is_same_object".to_owned(),
                reason: format_fail_comparison(actual, &expected)
            }
        }
}
