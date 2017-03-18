Galvanic-assert: Matcher-based assertions and expectations for easier testing
=============================================================================
[![Build Status](https://travis-ci.org/mindsbackyard/galvanic-assert.svg?branch=master)](https://travis-ci.org/mindsbackyard/galvanic-assert)
[![Documentation](https://docs.rs/galvanic-assert/badge.svg)](https://docs.rs/galvanic-assert)
[![Crates.io](https://img.shields.io/crates/v/galvanic-assert.svg)](https://crates.io/crates/galvanic-assert)

This crate provides a new assertion macros (`assert_that!`, `expect_that!`, `get_expectation_for!`) based on **matching predicates** (matchers) to
 * make **writing** asserts easier
 * make **reading** asserts comprehendable
 * easily **extend** the assertion framework
 * provide a large list **common matchers**
 * integrate with **galvanic-test** and **galvanic-mock** (both still in development ... stay tuned!)
 * be used with your favourite test framework

The crate will be part of **galvanic**---a complete test framework for **Rust**.

The 2-minute tutorial
---------------------
Each assertion has the form `assert_that!(SOME_VALUE, MATCHES_SOMETHING);`.
To check if some value satisfies some matching predicate, e.g., `less_than`, `contains_in_order`, `is_variant!`, ...; we can write something like the following to when operating on a single value:
```rust
#[macro_use]
extern crate galvanic_assert;
use galvanic_assert::matchers::*;

#[test]
fn expression_should_compute_correct_value {
    assert_that!(&1+2, eq(3));
    // or more wordy ...
    assert_that!(&1+2, is(eq(3)));
}
```

or assert properties of collections ...
```rust
use galvanic_assert::matchers::collection::*;
#[test]
fn expression_should_compute_correct_value {
    /// check for containment
    assert_that!(&vec![5,1,3,4,2], contains_in_any_order(vec![1,2,3,4,5]));
    // check for internal structure
    assert_that!(&vec![4,3,2,1], sorted_descending());
}
```

or of variants ...
```rust
enum Variants {
    First(i32),
    Second { x: i32, y: i32 },
    Third
}

#[test]
fn should_be_the_correct_variant {
    let var = Second { x: 1, y: 2 };
    assert_that!(&var, is_variant!(Variants::Second));
}
```

It is also possible to combine multiple matchers to create more expressive ones ...
```rust
#[test]
fn expression_should_compute_correct_value {
    // invert the meaning of a matcher
    assert_that!(&1+2, not(greater_than(3)));
    // join several matchers conjunctively
    assert_that!(&1+2, all_of!(greater_than(0), less_than(5)));
    // join several matchers disjunctively
    assert_that!(&1+2, any_of!(greater_than(5), less_than(5)));
}
```

If this is not enough you can write your own matchers, either as a closure ...
```rust
#[test]
fn expression_should_compute_correct_value {
    assert_that!(&1+2, |x| {
        let builder = MatchResultBuilder::for_("odd");
        if x % 2 == 1 { builder.matched() } else { builder.failed_because("result is not odd") }
    });
}
```

or if it is easy enough, as an expression ...
```rust
#[test]
fn expression_should_compute_correct_value {
    let val = 1 + 2;
    assert_that!(val % 2 == 1, otherwise "result is not odd");
}
```
If it is more complex implement the `Matcher` trait for some struct representing the state of the matcher.

Asserting positive things is good, but sometimes we expect that something goes horribly wrong ...
```rust
fn press_big_red_button() {
    panic!("shouldn't have done that ...");
}

#[test]
fn someting_should_go_wrong {
    assert_that!(press_big_red_button(), panics);
}
```

... except when it doesn't.
```rust
fn press_faulty_red_button() {
    return;
    panic!("shouldn't have done that ...");
}

#[test]
fn everything_is_fine {
    assert_that!(press_faulty_red_button(), does not panic);
}
```

Another 2-minutes for learning about expectations
-------------------------------------------------
An assertion is immediately check for correctness.
That means that any later assertion is executed and you might lose valuable information for debugging the error.
Basically you want as much information as possible to do that.
Therefore the the macros `expect_that!` and `get_expectation_for!` have been introduced.

Both state an expectation instead of an assertion in exactly the same way as `assert_that!`---so anything you learned from the last sectio still applies.
The condition is still checked at the point of specification but the inspection of the result is deferred to a later point in time.

The `expect_that!` macro defers the inspection of the result until the end of the current block:
```rust
{
    expect_that!(&1+1, equal_to(0));
    expect_that!(&1+1, less_than(4)); // is executed
}
expect_that!(1+1, panics); // is never executed as e1 panics
```

The `get_expectation_for!` macro allows for more fine grained control.
It returns an `Expectation` object which can be verified by calling `verify` ...
```rust
let e1 = get_expectation_for!(&1+1, equal_to(0));
let e2 = get_expectation_for!(&1+1, less_than(4)); // is executed
e1.verify();
let e3 = get_expectation_for!(&1+1, panics); // is never executed as e1 panics
```
... or will be automatically verified once the object goes out of scope.
```rust
{
    let e1 = get_expectation_for!(&1+1, equal_to(0));
    let e2 = get_expectation_for!(&1+1, less_than(4)); // is executed
}
let e3 = get_expectation_for!(1+1, panics); // is never executed as e1 panics
```

Not much more to say---have a look at the [documentation](https://docs.rs/galvanic-assert) and the growing list of matchers.

And remember ...
*only tested code is happy code!*

Contributions
-------------
Contributions are very welcome! (Please read and agree with the license.)

The list of included matchers is far from complete.
If you encounter a useful matcher please open an issue.
Check before if there's already a boolean predicate on the type itself, e.g., like `Option::is_none()`.
Those are already supported by the assertion macros and should only be included if the error message of the `Matcher` is substantially better than the default one.
If something is missing or broken, please open an issue and send (if you want to) a pull request.

For pull requests be sure to include test cases to avoid regressions.
Tests for `Matchers` should be added as integration tests as they test the integration with the assertion macros.
Have a look at the existing ones!
