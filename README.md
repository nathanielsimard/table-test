# Table Test
[![Build Status](https://travis-ci.org/nathanielsimard/table-test.svg?branch=master)](https://travis-ci.org/nathanielsimard/table-test)

This library aims to make table testing reliable in Rust. 
The main problem of table testing with basic Rust is `assert_eq!` calling `panic!`.
It means that when an assertion fails, then the rest of the test function is not executed.
In the case of a table test, it will result with potentially multiple use cases untested, making the output of that test unreliable.

## Usage

Specify this crate as `[dev-dependencies]`.

```toml
[dev-dependencies]
table_test = "*"
```

```rust
#[cfg(test)] // <-- not needed in integration tests
#[macro_use]
extern crate table_test;
```

The table iterator returns a tuple `(validator, input, expected)`.
If you have more than one input, just use a tuple of inputs.
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

![multiple inputs](assets/multiple_inputs.png)

As we can see, it would be easier to debug than with a normal `assert_eq!` output.
But the gain is when we work with something more complex.
The example [here](examples/mutable_struct.rs) test the changing name method and the result looks like this:

![mutable struct](assets/mutable_struct.png)
