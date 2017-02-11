#[macro_use]
extern crate galvanic_assert;

use galvanic_assert::*;
use galvanic_assert::matchers::*;

mod all_of {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(1, All::of(assertion_always_succeeds())
                            .and(assertion_always_succeeds())
        );
    }

    #[test]
    fn should_match_with_macro() {
        assert_that!(1, all_of!(assertion_always_succeeds(), assertion_always_succeeds()));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(1, All::of(assertion_always_succeeds())
                                .and(assertion_always_fails())
            ),
            panics
        );
    }
}

mod any_of {
    use super::*;

    #[test]
    fn should_match_if_none_fails() {
        assert_that!(1, Any::of(assertion_always_succeeds())
                            .or(assertion_always_succeeds())
        );
    }

    #[test]
    fn should_match_if_some_fails() {
        assert_that!(1, Any::of(assertion_always_succeeds())
                            .or(assertion_always_fails())
        );
    }

    #[test]
    fn should_match_with_macro() {
        assert_that!(1, any_of!(assertion_always_succeeds(), assertion_always_fails()));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(1, Any::of(assertion_always_fails())
                                .or(assertion_always_fails())
            ),
            panics
        );
    }
}
