#[macro_use]
extern crate galvanic_assert;

use galvanic_assert::*;
use galvanic_assert::matchers::*;

mod assert_panic {
    use super::*;

    #[test]
    fn should_assert_a_panic() {
        assert_that!(panic!("panic"), panics);
    }

    #[test]
    fn should_fail_to_assert_a_panic() {
        let panicked = std::panic::catch_unwind(|| {
            assert_that!(1+1, panics);
        });
        assert!(panicked.is_err());
    }
}

mod invariants {
    use super::*;

    #[test]
    fn assertion_should_succeed() {
        assert_that!(1, assertion_always_succeeds());
    }

    #[test]
    fn assertion_should_fail() {
        assert_that!(
            assert_that!(1, assertion_always_fails()),
            panics
        );
    }
}

mod not {
    use super::*;

    #[test]
    fn should_invert_success() {
        assert_that!(1, not(assertion_always_fails()));
    }

    #[test]
    fn should_invert_fail() {
        assert_that!(
            assert_that!(1, not(assertion_always_succeeds())),
            panics
        );
    }
}

mod eq {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(1, eq(1));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(1, eq(2)), panics
        );
    }
}

mod less_than {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(1, less_than(2));
        assert_that!(1, lt(2));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(1, less_than(1)),
            panics
        );
        assert_that!(
            assert_that!(1, lt(1)),
            panics
        );
    }
}

mod greater_than {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(1, greater_than(0));
        assert_that!(1, gt(0));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(1, greater_than(1)),
            panics
        );
        assert_that!(
            assert_that!(1, gt(1)),
            panics
        );
    }
}

mod less_than_or_equal {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(1, less_than_or_equal(2));
        assert_that!(1, less_than_or_equal(1));
        assert_that!(1, leq(2));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(1, less_than_or_equal(0)),
            panics
        );
        assert_that!(
            assert_that!(1, leq(0)),
            panics
        );
    }
}

mod greater_than_or_equal {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(1, greater_than_or_equal(0));
        assert_that!(1, greater_than_or_equal(1));
        assert_that!(1, geq(0));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(1, greater_than_or_equal(2)),
            panics
        );
        assert_that!(
            assert_that!(1, geq(2)),
            panics
        );
    }
}

mod close_to {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(3.14, close_to(3.14, 0.001));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(3.145, close_to(3.14, 0.001)),
            panics
        );
    }
}

mod same_object {
    use super::*;

    #[derive(Debug)]
    struct Foo;

    #[test]
    fn should_match() {
        let foo = Foo {};
        assert_that!(foo, same_object(&foo));
    }

    #[test]
    fn should_fail() {
        let foo1 = Foo {};
        let foo2 = Foo {};
        assert_that!(
            assert_that!(foo1, same_object(&foo2)),
            panics
        );
    }
}
