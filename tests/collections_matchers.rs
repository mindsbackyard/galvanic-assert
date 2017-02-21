#[macro_use]
extern crate galvanic_assert;

use galvanic_assert::*;
//use galvanic_assert::matchers::*;
use galvanic_assert::matchers::collection::*;

mod contains_in_any_order {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(vec![1,2,3,4], contains_in_any_order(vec![3,4,1,2]));
    }

    #[test]
    fn should_fail_due_to_unexpected_element() {
        assert_that!(
            assert_that!(vec![1,2,3,4,5], contains_in_any_order(vec![3,4,1,2])),
            panics
        );
    }

    #[test]
    fn should_fail_due_to_missing_element() {
        assert_that!(
            assert_that!(vec![1,2,4], contains_in_any_order(vec![3,4,1,2])),
            panics
        );
    }
}

mod contains_in_order {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(vec![1,2,3,4], contains_in_order(vec![1,2,3,4]));
    }

    #[test]
    fn should_fail_due_to_unexpected_element() {
        assert_that!(
            assert_that!(vec![1,2,3,4,5], contains_in_order(vec![1,2,3,4])),
            panics
        );
    }

    #[test]
    fn should_fail_due_to_missing_element() {
        assert_that!(
            assert_that!(vec![1,2,3], contains_in_order(vec![1,2,3,4])),
            panics
        );
    }

    #[test]
    fn should_fail_due_to_unordered_element() {
        assert_that!(
            assert_that!(vec![1,2,4,3], contains_in_order(vec![1,2,3,4])),
            panics
        );
    }
}

mod contained_in {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(3, contained_in(vec![1,2,3,4]));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(5, contained_in(vec![1,2,3,4])),
            panics
        );
    }
}

mod sorted_by {
    use super::*;

    #[test]
    fn should_match_single_element() {
        assert_that!(vec![1], sorted_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less));
    }

    #[test]
    fn should_match() {
        assert_that!(vec![1,2,3,4], sorted_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less));
    }

    #[test]
    fn should_match_all_eq() {
        assert_that!(vec![1,1,1,1], sorted_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less));
    }

    #[test]
    fn should_match_some_eq() {
        assert_that!(vec![1,2,2,3], sorted_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less));
    }

    #[test]
    fn should_match_eq_then_ordered() {
        assert_that!(vec![1,1,2,3], sorted_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less));
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(vec![1,2,5,4], sorted_by(|a: &i32, b: &i32| b.cmp(a), std::cmp::Ordering::Less)),
            panics
        );
    }

    #[test]
    fn should_fail_some_eq_then_unordered() {
        assert_that!(
            assert_that!(vec![1,2,2,1], sorted_by(|a: &i32, b: &i32| b.cmp(a), std::cmp::Ordering::Less)),
            panics
        );
    }
}

mod sorted_strictly_by {
    use super::*;

    #[test]
    fn should_match_single_element() {
        assert_that!(vec![1], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less));
    }

    #[test]
    fn should_match() {
        assert_that!(vec![1,2,3,4], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less));
    }

    #[test]
    fn should_fail_all_eq() {
        assert_that!(
            assert_that!(vec![1,1,1,1], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less)),
            panics
        );
    }

    #[test]
    fn should_fail_not_strict() {
        assert_that!(
            assert_that!(vec![1,2,2,3], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), std::cmp::Ordering::Less)),
            panics
        );
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(vec![1,2,5,4], sorted_strictly_by(|a: &i32, b: &i32| b.cmp(a), std::cmp::Ordering::Less)),
            panics
        );
    }
}

mod sorted_by_in_any_order {
    use super::*;

    #[test]
    fn should_match_single_element() {
        assert_that!(vec![1], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_sorted_ascending() {
        assert_that!(vec![1,2,2,4], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_sorted_descending() {
        assert_that!(vec![4,2,2,1], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_all_eq() {
        assert_that!(vec![1,1,1,1], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_eq_then_ordered() {
        assert_that!(vec![1,1,2,3], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(vec![1,2,5,4], sorted_by_in_any_order(|a: &i32, b: &i32| b.cmp(a))),
            panics
        );
    }

    #[test]
    fn should_fail_some_eq_then_unordered() {
        assert_that!(
            assert_that!(vec![1,2,2,1], sorted_by_in_any_order(|a: &i32, b: &i32| b.cmp(a))),
            panics
        );
    }
}

mod sorted_strictly_by_in_any_order {
    use super::*;

    #[test]
    fn should_match_single_element() {
        assert_that!(vec![1], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_sorted_strictly_ascending() {
        assert_that!(vec![1,2,3,4], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_sorted_strictly_descending() {
        assert_that!(vec![4,3,2,1], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_fail_all_eq() {
        assert_that!(
            assert_that!(vec![1,1,1,1], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b))),
            panics
        );
    }

    #[test]
    fn should_fail_not_strict() {
        assert_that!(
            assert_that!(vec![1,2,2,3], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b))),
            panics
        );
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(vec![1,2,5,4], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| b.cmp(a))),
            panics
        );
    }
}

mod sorted_ascending {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(vec![1,2,2,4], sorted_ascending());
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(vec![1,3,4,2], sorted_ascending()),
            panics
        );
    }

    #[test]
    fn should_fail_reversed() {
        assert_that!(
            assert_that!(vec![4,3,2,1], sorted_ascending()),
            panics
        );
    }
}

mod sorted_strictly_ascending {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(vec![1,2,3,4], sorted_strictly_ascending());
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(vec![1,3,4,2], sorted_strictly_ascending()),
            panics
        );
    }

    #[test]
    fn should_fail_not_strict() {
        assert_that!(
            assert_that!(vec![1,3,3,4], sorted_strictly_ascending()),
            panics
        );
    }

    #[test]
    fn should_fail_reversed() {
        assert_that!(
            assert_that!(vec![4,3,2,1], sorted_strictly_ascending()),
            panics
        );
    }
}

mod sorted_descending {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(vec![4,3,2,1], sorted_descending());
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(vec![4,1,2,3], sorted_descending()),
            panics
        );
    }

    #[test]
    fn should_fail_reversed() {
        assert_that!(
            assert_that!(vec![1,2,3,4], sorted_descending()),
            panics
        );
    }
}

mod sorted_strictly_descending {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(vec![4,3,2,1], sorted_strictly_descending());
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(vec![4,3,1,2], sorted_strictly_descending()),
            panics
        );
    }

    #[test]
    fn should_fail_not_strict() {
        assert_that!(
            assert_that!(vec![4,3,3,1], sorted_strictly_descending()),
            panics
        );
    }

    #[test]
    fn should_fail_reversed() {
        assert_that!(
            assert_that!(vec![1,2,3,4], sorted_strictly_descending()),
            panics
        );
    }
}

mod all_elements_satisfy {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(vec![2,4,6,8], all_elements_satisfy(|x| x % 2 == 0));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(vec![2,4,6,8], all_elements_satisfy(|x| x % 2 == 1)),
            panics
        );
    }
}

mod some_elements_satisfy {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(vec![2,3,6,8], some_elements_satisfy(|x| x % 2 == 1));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(vec![2,4,6,8], some_elements_satisfy(|x| x % 2 == 1)),
            panics
        );
    }
}
