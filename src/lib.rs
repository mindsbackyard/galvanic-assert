#![feature(conservative_impl_trait)]

use std::fmt;

macro_rules! assert_that {
    ( $actual: expr, $matcher: expr ) => {
        match $matcher.check(&$actual) {
            MatchResult::Match => { },
            MatchResult::FailWithReason{matcher, reason} => {
                panic!("\nFailed assertion of matcher: {}\n  {}\n", matcher, reason)
            },
            MatchResult::FailWithComparison{matcher, expected, actual} => {
                panic!("\nFailed assertion of matcher {}\n  Expected: {}\n  Got: {}\n", matcher, expected, actual);
            }
        }
    }
}


pub trait Matcher<T> {
    fn check(&mut self, actual: &T) -> MatchResult;
}

impl<T, F> Matcher<T> for F
where F: FnMut(&T) -> MatchResult {
    fn check(&mut self, actual: &T) -> MatchResult {
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


pub fn format_fail_reason(reason: &str) -> String {
    format!("  Because: {}", reason)
}

pub fn format_fail_comparison<T>(actual: T, expected: T) -> String
where T: fmt::Debug {
    format!("  Expected: {:?}\n  Got: {:?}", expected, actual)
}

pub mod matchers;
