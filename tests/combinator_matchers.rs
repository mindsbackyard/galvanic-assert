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

mod combining_combinators {
    use super::*;

    #[test]
    fn should_match_combined_any_all() {
        let x = 1;
        assert_that!(&x, any_of!(all_of!(assertion_always_succeeds(), assertion_always_succeeds()),
                                 all_of!(assertion_always_succeeds(), assertion_always_succeeds())
        ));
    }

    #[test]
    fn should_match_combined_all_any() {
        let x = 1;
        assert_that!(&x, all_of!(any_of!(assertion_always_succeeds(), assertion_always_succeeds()),
                                 any_of!(assertion_always_fails(), assertion_always_succeeds())
        ));
    }

    #[test]
    fn should_match_combined_not_any() {
        let x = 1;
        assert_that!(&x, not(any_of!(assertion_always_fails(), assertion_always_fails())));
    }
}

mod combining_variant_matchers {
    use galvanic_assert::matchers::*;

    #[allow(dead_code)]
    enum Variants {
        First,
        Second,
        Third
    }

    #[test]
    fn should_match() {
        let x = Variants::Second;
        assert_that!(&x, any_of!(is_variant!(Variants::First),
                                 is_variant!(Variants::Second)
        ));
    }
}
