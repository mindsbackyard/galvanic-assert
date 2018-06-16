/* Copyright 2017 Christopher Bacher
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#[macro_use]
extern crate galvanic_assert;

use galvanic_assert::matchers::collection::*;

mod contains_in_any_order {
    use super::contains_in_any_order;

    #[test]
    fn should_match() {
        assert_that!(&vec![1,2,3,4], contains_in_any_order(vec![3,4,1,2]));
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), contains_in_any_order::<i32, Vec<i32>, Vec<i32>>(Vec::new()));
    }

    #[test]
    fn should_fail_due_to_unexpected_element() {
        assert_that!(
            assert_that!(&vec![1,2,3,4,5], contains_in_any_order(vec![3,4,1,2])),
            panics
        );
    }

    #[test]
    fn should_fail_due_to_missing_element() {
        assert_that!(
            assert_that!(&vec![1,2,4], contains_in_any_order(vec![3,4,1,2])),
            panics
        );
    }
}

mod contains_in_order {
    use super::contains_in_order;

    #[test]
    fn should_match() {
        assert_that!(&vec![1,2,3,4], contains_in_order(vec![1,2,3,4]));
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), contains_in_order::<i32, Vec<i32>, Vec<i32>>(Vec::new()));
    }

    #[test]
    fn should_fail_due_to_unexpected_element() {
        assert_that!(
            assert_that!(&vec![1,2,3,4,5], contains_in_order(vec![1,2,3,4])),
            panics
        );
    }

    #[test]
    fn should_fail_due_to_missing_element() {
        assert_that!(
            assert_that!(&vec![1,2,3], contains_in_order(vec![1,2,3,4])),
            panics
        );
    }

    #[test]
    fn should_fail_due_to_unordered_element() {
        assert_that!(
            assert_that!(&vec![1,2,4,3], contains_in_order(vec![1,2,3,4])),
            panics
        );
    }
}

mod contains_subset {
    use super::contains_subset;

    #[test]
    fn should_match() {
        assert_that!(&vec![1,2,3,4,5], contains_subset(vec![3,4,1,2]));
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), contains_subset::<i32, Vec<i32>, Vec<i32>>(Vec::new()));
    }

    #[test]
    fn should_match_empty_subset() {
        assert_that!(&vec![1,2,3,4], contains_subset::<i32, Vec<i32>, Vec<i32>>(Vec::new()));
    }

    #[test]
    fn should_fail_due_to_missing_element() {
        assert_that!(
            assert_that!(&vec![1,2,4], contains_subset(vec![3,4,1,2])),
            panics
        );
    }
}

mod contained_in {
    use super::contained_in;

    #[test]
    fn should_match() {
        assert_that!(&3, contained_in(vec![1,2,3,4]));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&5, contained_in(vec![1,2,3,4])),
            panics
        );
    }
}

mod sorted_by {
    use super::sorted_by;
    use std::cmp::Ordering;

    #[test]
    fn should_match_single_element() {
        assert_that!(&vec![1], sorted_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), sorted_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_match() {
        assert_that!(&vec![1,2,3,4], sorted_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_match_all_eq() {
        assert_that!(&vec![1,1,1,1], sorted_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_match_some_eq() {
        assert_that!(&vec![1,2,2,3], sorted_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_match_eq_then_ordered() {
        assert_that!(&vec![1,1,2,3], sorted_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(&vec![1,2,5,4], sorted_by(|a: &i32, b: &i32| b.cmp(a), Ordering::Less)),
            panics
        );
    }

    #[test]
    fn should_fail_some_eq_then_unordered() {
        assert_that!(
            assert_that!(&vec![1,2,2,1], sorted_by(|a: &i32, b: &i32| b.cmp(a), Ordering::Less)),
            panics
        );
    }
}

mod sorted_strictly_by {
    use super::sorted_strictly_by;
    use std::cmp::Ordering;

    #[test]
    fn should_match_single_element() {
        assert_that!(&vec![1], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_match() {
        assert_that!(&vec![1,2,3,4], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
    }

    #[test]
    fn should_fail_all_eq() {
        assert_that!(
            assert_that!(&vec![1,1,1,1], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less)),
            panics
        );
    }

    #[test]
    fn should_fail_not_strict() {
        assert_that!(
            assert_that!(&vec![1,2,2,3], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less)),
            panics
        );
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(&vec![1,2,5,4], sorted_strictly_by(|a: &i32, b: &i32| b.cmp(a), Ordering::Less)),
            panics
        );
    }
}

mod sorted_by_in_any_order {
    use super::sorted_by_in_any_order;

    #[test]
    fn should_match_single_element() {
        assert_that!(&vec![1], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_sorted_ascending() {
        assert_that!(&vec![1,2,2,4], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_sorted_descending() {
        assert_that!(&vec![4,2,2,1], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_all_eq() {
        assert_that!(&vec![1,1,1,1], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_eq_then_ordered() {
        assert_that!(&vec![1,1,2,3], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(&vec![1,2,5,4], sorted_by_in_any_order(|a: &i32, b: &i32| b.cmp(a))),
            panics
        );
    }

    #[test]
    fn should_fail_some_eq_then_unordered() {
        assert_that!(
            assert_that!(&vec![1,2,2,1], sorted_by_in_any_order(|a: &i32, b: &i32| b.cmp(a))),
            panics
        );
    }
}

mod sorted_strictly_by_in_any_order {
    use super::sorted_strictly_by_in_any_order;

    #[test]
    fn should_match_single_element() {
        assert_that!(&vec![1], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_sorted_strictly_ascending() {
        assert_that!(&vec![1,2,3,4], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_match_sorted_strictly_descending() {
        assert_that!(&vec![4,3,2,1], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
    }

    #[test]
    fn should_fail_all_eq() {
        assert_that!(
            assert_that!(&vec![1,1,1,1], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b))),
            panics
        );
    }

    #[test]
    fn should_fail_not_strict() {
        assert_that!(
            assert_that!(&vec![1,2,2,3], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b))),
            panics
        );
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(&vec![1,2,5,4], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| b.cmp(a))),
            panics
        );
    }
}

mod sorted_ascending {
    use super::sorted_ascending;

    #[test]
    fn should_match() {
        assert_that!(&vec![1,2,2,4], sorted_ascending());
    }

    #[test]
    fn should_match_empty_collection() {
        let v: Vec<i32> = Vec::new();
        assert_that!(&v, sorted_ascending());
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(&vec![1,3,4,2], sorted_ascending()),
            panics
        );
    }

    #[test]
    fn should_fail_reversed() {
        assert_that!(
            assert_that!(&vec![4,3,2,1], sorted_ascending()),
            panics
        );
    }
}

mod sorted_strictly_ascending {
    use super::sorted_strictly_ascending;

    #[test]
    fn should_match() {
        assert_that!(&vec![1,2,3,4], sorted_strictly_ascending());
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), sorted_strictly_ascending::<i32, Vec<i32>>());
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(&vec![1,3,4,2], sorted_strictly_ascending()),
            panics
        );
    }

    #[test]
    fn should_fail_not_strict() {
        assert_that!(
            assert_that!(&vec![1,3,3,4], sorted_strictly_ascending()),
            panics
        );
    }

    #[test]
    fn should_fail_reversed() {
        assert_that!(
            assert_that!(&vec![4,3,2,1], sorted_strictly_ascending()),
            panics
        );
    }
}

mod sorted_descending {
    use super::sorted_descending;

    #[test]
    fn should_match() {
        assert_that!(&vec![4,3,2,1], sorted_descending());
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), sorted_descending::<i32, Vec<i32>>());
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&vec![4,1,2,3], sorted_descending()),
            panics
        );
    }

    #[test]
    fn should_fail_reversed() {
        assert_that!(
            assert_that!(&vec![1,2,3,4], sorted_descending()),
            panics
        );
    }
}

mod sorted_strictly_descending {
    use super::sorted_strictly_descending;

    #[test]
    fn should_match() {
        assert_that!(&vec![4,3,2,1], sorted_strictly_descending());
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), sorted_strictly_descending::<i32, Vec<i32>>());
    }

    #[test]
    fn should_fail_unordered() {
        assert_that!(
            assert_that!(&vec![4,3,1,2], sorted_strictly_descending()),
            panics
        );
    }

    #[test]
    fn should_fail_not_strict() {
        assert_that!(
            assert_that!(&vec![4,3,3,1], sorted_strictly_descending()),
            panics
        );
    }

    #[test]
    fn should_fail_reversed() {
        assert_that!(
            assert_that!(&vec![1,2,3,4], sorted_strictly_descending()),
            panics
        );
    }
}

mod all_elements_satisfy {
    use super::all_elements_satisfy;

    #[test]
    fn should_match() {
        assert_that!(&vec![2,4,6,8], all_elements_satisfy(|x| x % 2 == 0));
    }

    #[test]
    fn should_match_empty_collection() {
        assert_that!(&Vec::new(), all_elements_satisfy(|x| x % 2 == 0));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&vec![2,4,6,8], all_elements_satisfy(|x| x % 2 == 1)),
            panics
        );
    }
}

mod some_elements_satisfy {
    use super::some_elements_satisfy;

    #[test]
    fn should_match() {
        assert_that!(&vec![2,3,6,8], some_elements_satisfy(|x| x % 2 == 1));
    }

    #[test]
    fn should_fail_empty_collection() {
        assert_that!(
            assert_that!(&Vec::new(), some_elements_satisfy(|x| x % 2 == 0)),
            panics
        );
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&vec![2,4,6,8], some_elements_satisfy(|x| x % 2 == 1)),
            panics
        );
    }
}

mod has_entry {
    mod ordered_map {
        use std::collections::BTreeMap;
        use super::super::has_entry;

        #[test]
        fn should_match() {
            let mut map = BTreeMap::<i32,i32>::new();
            map.insert(1, 2);

            assert_that!(&map, has_entry(1, 2));
        }

        #[test]
        fn should_fail_due_to_missing_key() {
            let map = BTreeMap::<i32,i32>::new();

            assert_that!(
                assert_that!(&map, has_entry(1, 2)),
                panics
            );
        }

        #[test]
        fn should_fail_due_to_wrong_entry() {
            let mut map = BTreeMap::<i32,i32>::new();
            map.insert(1, 1);

            assert_that!(
                assert_that!(&map, has_entry(1, 2)),
                panics
            );
        }
    }

    mod hash_map {
        use std::collections::HashMap;
        use super::super::has_entry;

        #[test]
        fn should_match() {
            let mut map = HashMap::<i32,i32>::new();
            map.insert(1, 2);

            assert_that!(&map, has_entry(1, 2));
        }

        #[test]
        fn should_fail_due_to_missing_key() {
            let map = HashMap::<i32,i32>::new();

            assert_that!(
                assert_that!(&map, has_entry(1, 2)),
                panics
            );
        }

        #[test]
        fn should_fail_due_to_wrong_entry() {
            let mut map = HashMap::<i32,i32>::new();
            map.insert(1, 1);

            assert_that!(
                assert_that!(&map, has_entry(1, 2)),
                panics
            );
        }
    }
}

mod has_key {
    mod ordered_map {
        use std::collections::BTreeMap;
        use super::super::has_key;

        #[test]
        fn should_match() {
            let mut map = BTreeMap::<i32,i32>::new();
            map.insert(1, 2);

            assert_that!(&map, has_key(1));
        }

        #[test]
        fn should_fail_due_to_missing_key() {
            let mut map = BTreeMap::<i32,i32>::new();
            map.insert(1, 2);

            assert_that!(
                assert_that!(&map, has_key(2)),
                panics
            );
        }
    }

    mod hash_map {
        use std::collections::HashMap;
        use super::super::has_key;

        #[test]
        fn should_match() {
            let mut map = HashMap::<i32,i32>::new();
            map.insert(1, 2);

            assert_that!(&map, has_key(1));
        }

        #[test]
        fn should_fail_due_to_missing_key() {
            let mut map = HashMap::<i32,i32>::new();
            map.insert(1, 2);

            assert_that!(
                assert_that!(&map, has_key(2)),
                panics
            );
        }
    }
}

mod has_value {
    mod ordered_map {
        use std::collections::BTreeMap;
        use super::super::has_value;

        #[test]
        fn should_match() {
            let mut map = BTreeMap::<i32, i32>::new();
            map.insert(1, 2);

            assert_that!(&map, has_value(2));
        }

        #[test]
        fn should_fail_due_to_missing_key() {
            let mut map = BTreeMap::<i32, i32>::new();
            map.insert(1, 2);

            assert_that!(
                assert_that!(&map, has_value(1)),
                panics
            );
        }
    }

    mod hash_map {
        use std::collections::HashMap;
        use super::super::has_value;

        #[test]
        fn should_match() {
            let mut map = HashMap::<i32, i32>::new();
            map.insert(1, 2);

            assert_that!(&map, has_value(2));
        }

        #[test]
        fn should_fail_due_to_missing_key() {
            let mut map = HashMap::<i32, i32>::new();
            map.insert(1, 2);

            assert_that!(
                assert_that!(&map, has_value(1)),
                panics
            );
        }
    }
}
