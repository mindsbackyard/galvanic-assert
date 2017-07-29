/* Copyright 20&17 Christopher Bacher
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

use std::result::Result;

mod assert_panic {
    use super::*;

    #[test]
    fn should_assert_a_panic() {
        assert_that!(panic!("panic"), panics);
    }

    #[test]
    fn should_fail_to_assert_a_panic() {
        let panicked = std::panic::catch_unwind(|| {
            assert_that!(&1+1, panics);
        });
        assert!(panicked.is_err());
    }
}

mod assert_does_not_panic {
    use super::*;

    #[test]
    fn should_assert_that_no_panic_occurred() {
        assert_that!(&1+1, does not panic);
    }

    #[test]
    fn should_fail_to_assert_a_panic() {
        let panicked = std::panic::catch_unwind(|| {
            assert_that!(panic!("panic"), does not panic);
        });
        assert!(panicked.is_err());
    }
}

mod assert_expression {
    use super::*;

    #[test]
    fn should_assert_an_expression_to_be_true() {
        let ok: Result<i32,i32> = Ok(4);
        assert_that!(ok.is_ok());
    }

    #[test]
    fn should_fail_to_assert_an_expression_to_be_true() {
        let err: Result<i32,i32> = Err(4);
        assert_that!(
            assert_that!(err.is_ok()),
            panics
        );
    }

    #[test]
    fn should_assert_an_expression_to_be_true_with_reason() {
        let ok: Result<i32,i32> = Ok(4);
        assert_that!(ok.is_ok(), otherwise "ok is Err");
    }

    #[test]
    fn should_fail_to_assert_an_expression_to_be_true_with_reason() {
        let err: Result<i32,i32> = Err(4);
        assert_that!(
            assert_that!(err.is_ok(), otherwise "err is Ok"),
            panics
        );
    }
}

mod invariants {
    use super::*;

    #[test]
    fn assertion_should_succeed() {
        assert_that!(&1, assertion_always_succeeds());
    }

    #[test]
    fn assertion_should_fail() {
        assert_that!(
            assert_that!(&1, assertion_always_fails()),
            panics
        );
    }
}

mod not {
    use super::*;

    #[test]
    fn should_invert_success() {
        assert_that!(&1, not(assertion_always_fails()));
    }

    #[test]
    fn should_invert_fail() {
        assert_that!(
            assert_that!(&1, not(assertion_always_succeeds())),
            panics
        );
    }
}

mod eq {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(&1, eq(1));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&1, eq(2)), panics
        );
    }
}

mod less_than {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(&1, less_than(2));
        assert_that!(&1, lt(2));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&1, less_than(1)),
            panics
        );
        assert_that!(
            assert_that!(&1, lt(1)),
            panics
        );
    }
}

mod greater_than {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(&1, greater_than(0));
        assert_that!(&1, gt(0));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&1, greater_than(1)),
            panics
        );
        assert_that!(
            assert_that!(&1, gt(1)),
            panics
        );
    }
}

mod less_than_or_equal {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(&1, less_than_or_equal(2));
        assert_that!(&1, less_than_or_equal(1));
        assert_that!(&1, leq(2));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&1, less_than_or_equal(0)),
            panics
        );
        assert_that!(
            assert_that!(&1, leq(0)),
            panics
        );
    }
}

mod greater_than_or_equal {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(&1, greater_than_or_equal(0));
        assert_that!(&1, greater_than_or_equal(1));
        assert_that!(&1, geq(0));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&1, greater_than_or_equal(2)),
            panics
        );
        assert_that!(
            assert_that!(&1, geq(2)),
            panics
        );
    }
}

mod close_to {
    use super::*;

    #[test]
    fn should_match() {
        assert_that!(&3.14, close_to(3.14, 0.001));
    }

    #[test]
    fn should_fail() {
        assert_that!(
            assert_that!(&2.2, close_to(3.14, 0.001)),
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
        assert_that!(&foo, same_object(&foo));
    }

    #[test]
    fn should_fail() {
        let foo1 = Foo {};
        let foo2 = Foo {};
        assert_that!(
            assert_that!(&foo1, same_object(&foo2)),
            panics
        );
    }
}


#[cfg(structural_matchers)]
mod has_structure {
    use super::*;

    mod struct_like {
        use super::*;
        struct Foo { x: i32, y: f64 }

        #[test]
        fn should_match() {
            let foo = Foo { x: 12, y: 23.4 };
            assert_that!(&foo, structure!(Foo {
                x: eq(12),
                y: lt(25.0)
            }));
        }

        #[test]
        fn should_match_incomplete_field_list() {
            let foo = Foo { x: 12, y: 23.4 };
            assert_that!(&foo, structure!(Foo {
                y: lt(25.0)
            }));
        }

        #[test]#[should_panic]
        fn should_fail() {
            let foo = Foo { x: 12, y: 23.4 };
            assert_that!(&foo, structure!(Foo {
                x: eq(13),
                y: gt(25.0)
            }));
        }
    }

    mod tuple_like {
        use super::*;
        struct Bar(i32, f64);

        #[test]
        fn should_match() {
            let bar = Bar(12, 23.4);
            assert_that!(&bar, structure!(Bar [eq(12), lt(25.0)] ));
        }

        #[test]#[should_panic]
        fn should_fail() {
            let bar = Bar(12, 23.4);
            assert_that!(&bar, structure!(Bar [eq(12), gt(25.0)] ));
        }
    }

    mod enum_like {
        use super::*;

        enum Baz {
            Var1 { x: i32, y: f64 },
            Var2(i32, f64)
        }

        #[test]
        fn should_match() {
            let var1 = Baz::Var1 { x: 12, y: 23.4 };
            assert_that!(&var1, structure!(Baz::Var1 {
                x: eq(12),
                y: lt(25.0)
            }));

            let var2 = Baz::Var2(12, 23.4);
            assert_that!(&var2, structure!(Baz::Var2 [eq(12), lt(25.0)] ));
        }

        #[test]
        fn should_fail() {
            let var1 = Baz::Var1 { x: 12, y: 23.4 };
            assert_that!(
                assert_that!(&var1, structure!(Baz::Var1 {
                    x: eq(13),
                    y: gt(25.0)
                })),
                panics
            );

            assert_that!(
                assert_that!(&var1, structure!(Baz::Var2 [any_value(), any_value()])),
                panics
            );

            let var2 = Baz::Var2(12, 23.4);
            assert_that!(
                assert_that!(&var2, structure!(Baz::Var2 [eq(13), gt(25.0)] )),
                panics
            );
            assert_that!(
                assert_that!(&var2, structure!(Baz::Var1 { x: any_value(), y: any_value() } )),
                panics
            );
        }
    }
}
