#![feature(conservative_impl_trait)]
#![feature(discriminant_value)]

use std::fmt;

#[macro_export]
macro_rules! assert_that {
    ( $actual: expr, panics ) => {
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_ok() {
            panic!("\nFailed assertion; expected expression to panic")
        }
    };
    ( $actual: expr, does not panic ) => {
        let result = std::panic::catch_unwind(|| { $actual; });
        if result.is_err() {
            panic!("\nFailed assertion; expected expression to panic")
        }
    };
    ( $actual: expr ) => {
        if !$actual {
            panic!("\nFailed assertion; expression '{}' is not true", stringify!($actual));
        }
    };
    ( $actual: expr, $matcher: expr ) => {
        match $matcher.check($actual) {
            MatchResult::Matched { .. } => { },
            MatchResult::Failed { name, reason } => {
                panic!("\nFailed assertion of matcher: {}\n{}", name, reason)
            }
        }
    };
}


pub trait Matcher<T> {
    fn check(&self, actual: T) -> MatchResult;
}

impl<T, F> Matcher<T> for F
where F: Fn(T) -> MatchResult {
    fn check(&self, actual: T) -> MatchResult {
        self(actual)
    }
}


pub enum MatchResult {
    Matched {
        name: String
    },
    Failed {
        name: String,
        reason: String
    }
}


pub struct MatchResultBuilder {
    matcher_name: String
}

impl MatchResultBuilder {
    pub fn new() -> MatchResultBuilder {
        MatchResultBuilder {
            matcher_name: "_unknown_".to_owned()
        }
    }

    pub fn for_(name: &str) -> MatchResultBuilder {
        MatchResultBuilder {
            matcher_name: name.to_owned()
        }
    }

    pub fn matched(self) -> MatchResult {
        MatchResult::Matched { name: self.matcher_name }
    }

    pub fn failed_because(self, reason: &str) -> MatchResult {
        MatchResult::Failed {
            name: self.matcher_name,
            reason: format!("  Because: {}", reason)
        }
    }

    pub fn failed_comparison<T: fmt::Debug>(self, actual: &T, expected: &T) -> MatchResult {
        MatchResult::Failed {
            name: self.matcher_name,
            reason: format!("  Expected: {:?}\n  Got: {:?}", expected, actual)
        }
    }
}

pub mod matchers;
