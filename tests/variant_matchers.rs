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

#[allow(dead_code)]
#[derive(Debug)]
enum MyEnum {
    Var1,
    Var2 {x: i32}
}

mod is_variant {
    use super::*;

    #[test]
    fn should_succeed() {
        assert_that!(&MyEnum::Var1, is_variant!(MyEnum::Var1));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&MyEnum::Var1, is_variant!(MyEnum::Var2)),
            panics
        );
    }
}

mod maybe_some {
    use galvanic_assert::matchers::equal_to;
    use galvanic_assert::matchers::variant::maybe_some;

    #[test]
    fn should_succeed() {
        let maybe_int = Some(2);
        assert_that!(&maybe_int, maybe_some(equal_to(2)));
    }

    #[test]
    #[should_panic]
    fn should_fail_because_of_none_value() {
        let maybe_int = None;
        assert_that!(&maybe_int, maybe_some(equal_to(2)));
    }

    #[test]
    #[should_panic]
    fn should_fail_because_nested_matcher_fails() {
        let maybe_int = Some(3);
        assert_that!(&maybe_int, maybe_some(equal_to(2)));
    }
}

mod maybe_ok {
    use galvanic_assert::matchers::equal_to;
    use galvanic_assert::matchers::variant::maybe_ok;

    #[test]
    fn should_succeed() {
        let maybe_int: Result<i32, String> = Ok(2);
        assert_that!(&maybe_int, maybe_ok(equal_to(2)));
    }

    #[test]
    #[should_panic]
    fn should_fail_because_of_none_value() {
        let maybe_int: Result<i32, String> = Err("Failed".to_owned());
        assert_that!(&maybe_int, maybe_ok(equal_to(2)));
    }

    #[test]
    #[should_panic]
    fn should_fail_because_nested_matcher_fails() {
        let maybe_int: Result<i32, String> = Ok(3);
        assert_that!(&maybe_int, maybe_ok(equal_to(2)));
    }
}

mod maybe_err {
    use galvanic_assert::matchers::equal_to;
    use galvanic_assert::matchers::variant::maybe_err;

    #[test]
    fn should_succeed() {
        let maybe_int: Result<String, i32> = Err(2);
        assert_that!(&maybe_int, maybe_err(equal_to(2)));
    }

    #[test]
    #[should_panic]
    fn should_fail_because_of_none_value() {
        let maybe_int: Result<String, i32> = Ok("Ok".to_owned());
        assert_that!(&maybe_int, maybe_err(equal_to(2)));
    }

    #[test]
    #[should_panic]
    fn should_fail_because_nested_matcher_fails() {
        let maybe_int: Result<String, i32> = Err(3);
        assert_that!(&maybe_int, maybe_err(equal_to(2)));
    }
}
