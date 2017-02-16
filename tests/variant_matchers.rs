#[macro_use]
extern crate galvanic_assert;

use galvanic_assert::*;
use galvanic_assert::matchers::*;
use galvanic_assert::matchers::variant::*;

#[derive(Debug)]
enum MyEnum {
    Var1,
    Var2 {x: i32}
}

mod is_variant {
    use super::*;

    #[test]
    fn should_succeed() {
        let x = MyEnum::Var1;
        assert_that!(x, is_variant!(MyEnum, MyEnum::Var1));
    }

    #[test]
    fn should_fail() {
        let x = MyEnum::Var1;
        assert_that!(
            assert_that!(x, is_variant!(MyEnum, MyEnum::Var2)),
            panics
        );
    }
}

mod same_variant_as {
    use super::*;

    #[test]
    fn should_succeed() {
        let x = MyEnum::Var1;
        let expected = MyEnum::Var1;

        assert_that!(x, same_variant_as(&expected));
    }

    #[test]
    fn should_fail() {
        let x = MyEnum::Var1;
        let expected = MyEnum::Var2{ x: 12 };
        assert_that!(
            assert_that!(x, same_variant_as(&expected)),
            panics
        );
    }
}
