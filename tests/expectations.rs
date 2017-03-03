#[macro_use]
extern crate galvanic_assert;

use galvanic_assert::*;
use galvanic_assert::matchers::*;

mod get_expectation_for_expression {
    use super::*;

    #[test]
    fn should_get_an_expectation_asserting_an_expression_to_be_true() {
        #[allow(unused_variables)]
        let e = get_expectation_for!(1 == 1);
    }

    #[test]
    fn should_get_an_expectation_asserting_an_expression_to_be_true_and_verify_manually() {
        let e = get_expectation_for!(1 == 1);
        e.verify()
    }

    #[test]
    fn should_get_multiple_expectations_asserting_an_expression_to_be_true() {
        #[allow(unused_variables)]
        let e1 = get_expectation_for!(1 == 1);
        #[allow(unused_variables)]
        let e2 = get_expectation_for!(1 == 1, otherwise "failed");
    }

    #[test]
    #[should_panic]
    fn should_get_an_expectation_failing_to_assert_an_expression_to_be_true() {
        #[allow(unused_variables)]
        let e = get_expectation_for!(1 != 1);
    }

    #[test]
    #[should_panic]
    fn should_get_an_expectation_failing_to_assert_an_expression_to_be_true_and_verify_manually() {
        let e = get_expectation_for!(1 != 1);
        e.verify()
    }

    #[test]
    fn should_get_multiple_expectations_failing_to_assert_an_expression_to_be_true() {
        let e1 = get_expectation_for!(1 != 1);
        let e2 = get_expectation_for!(1 != 1, otherwise "failed");
        assert_that!(e1.verify(), panics);
        assert_that!(e2.verify(), panics);
    }

    #[test]
    fn should_get_multiple_expectations_some_failing_to_assert_an_expression_to_be_true() {
        let e1 = get_expectation_for!(1 != 1);
        let e2 = get_expectation_for!(1 == 1);
        assert_that!(e1.verify(), panics);
        assert_that!(e2.verify(), does not panic);
    }
}

mod get_expectation_for_panic {
    use super::*;

    #[test]
    fn should_get_an_expectation_asserting_an_expression_to_panic() {
        #[allow(unused_variables)]
        let e = get_expectation_for!(panic!("panic"), panics);
    }

    #[test]
    #[should_panic]
    fn should_get_an_expectation_failing_to_assert_an_expression_to_panic() {
        #[allow(unused_variables)]
        let e = get_expectation_for!(1 == 1, panics);
    }
}

mod get_expectation_for_no_panic {
    use super::*;

    #[test]
    fn should_get_an_expectation_asserting_an_expression_to_not_panic() {
        #[allow(unused_variables)]
        let e = get_expectation_for!(1 == 1, does not panic);
    }

    #[test]
    #[should_panic]
    fn should_get_an_expectation_failing_to_assert_an_expression_to_panic() {
        #[allow(unused_variables)]
        let e = get_expectation_for!(panic!("panic"), does not panic);
    }
}

mod get_expectation_for_matcher {
    use super::*;

    #[test]
    fn should_get_an_expectation_asserting_a_matcher_to_succeed() {
        #[allow(unused_variables)]
        let e = get_expectation_for!(1, assertion_always_succeeds());
    }

    #[test]
    #[should_panic]
    fn should_get_an_expectation_asserting_a_matcher_to_fail() {
        #[allow(unused_variables)]
        let e = get_expectation_for!(1, assertion_always_fails());
    }
}
