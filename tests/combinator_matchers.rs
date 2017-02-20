#[macro_use]
extern crate galvanic_assert;

use galvanic_assert::*;
use galvanic_assert::matchers::*;

mod all_of {
    use super::*;

    #[test]
    fn should_match() {
        let x = 1;
        assert_that!(&x, All::of(assertion_always_succeeds())
                             .and(assertion_always_succeeds())
        );
    }

    #[test]
    fn should_match_with_macro() {
        let x = 1;
        assert_that!(&x, all_of!(assertion_always_succeeds(), assertion_always_succeeds()));
    }

    #[test]
    fn should_fail() {
        let x = 1;
        assert_that!(
            assert_that!(&x, All::of(assertion_always_succeeds())
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
        let x = 1;
        assert_that!(&x, Any::of(assertion_always_succeeds())
                             .or(assertion_always_succeeds())
        );
    }

    #[test]
    fn should_match_if_some_fails() {
        let x = 1;
        assert_that!(&x, Any::of(assertion_always_succeeds())
                             .or(assertion_always_fails())
        );
    }

    #[test]
    fn should_match_with_macro() {
        let x = 1;
        assert_that!(&x, any_of!(assertion_always_succeeds(), assertion_always_fails()));
    }

    #[test]
    fn should_fail() {
        let x = 1;
        assert_that!(
            assert_that!(&x, Any::of(assertion_always_fails())
                                 .or(assertion_always_fails())
            ),
            panics
        );
    }
}
