# Table Test
[![Build Status](https://travis-ci.org/nathanielsimard/table-test.svg?branch=master)](https://travis-ci.org/nathanielsimard/table-test)
[![codecov.io](https://codecov.io/gh/nathanielsimard/table-test/coverage.svg?branch=master)](https://codecov.io/gh/nathanielsimard/table-test)
[![Current Crates.io Version](https://img.shields.io/crates/v/table-test.svg)](https://crates.io/crates/table-test)

This library aims to make table testing reliable in Rust. 
The main problem of table testing with basic Rust is `assert_eq!` calling `panic!`.
It means that when an assertion fails, then the rest of the test function is not executed.
In the case of a table test, it will result with potentially multiple use cases untested, making the output of that test unreliable.

## Usage

Specify this crate as `[dev-dependencies]`.

```toml
[dev-dependencies]
table-test = "0.2.0"
```

```rust
#[cfg(test)] // <-- not needed in integration tests
#[macro_use]
extern crate table_test;
```

The table iterator returns a tuple `(test_case, input, expected)`.
If you have more than one input, just use a tuple of inputs.
The `test_case` allows you to add comments like `given` `when` and `then`, but also `description` and `custom` giving you the freedom to log your tests the best way possible.

## Examples

If we make a simple test for an `add` function that takes two values as input:

```rust
#[test]
fn test_add() {
    let table = vec![
        ((1, 2), 3),
        ((2, 5), 7),
        ((0, 0), 0),
        ((0, 1), 1),
        ((2, 2), 4),
    ];

    for (validator, (input_1, input_2), expected) in table_test!(table) {
        let actual = add(input_1, input_2);

        validator
            .given(&format!("{}, {}", input_1, input_2))
            .when("add")
            .then(&format!("it should be {}", expected))
            .assert_eq(expected, actual);
    }
}
```
If we make a mistake in the implementation of the `add` function and multiplying instead, then the output will look like this:

![multiple inputs](examples_outputs/multiple_inputs.png)

As we can see, it would be easier to debug than with a normal `assert_eq!` output.
But the gain is when we work with something more complex.
The example [here](examples/mutable_struct.rs) test the changing name method and the result looks like this:

![mutable struct](examples_outputs/mutable_struct.png)

More examples can be found in the examples folder.