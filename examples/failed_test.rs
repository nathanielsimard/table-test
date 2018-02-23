#[macro_use]
extern crate table_test;

fn main() {
    let table = table_test!(
        concat,
        [
            (("Hello ", "World"), "Hello World"),
            (("Hello ", " "), "Hello  "),
            (("Hello", ""), "Hello"),
        ]
    );

    for (validator, (input_1, input_2), expected) in table {
        let actual = concat(input_1, input_2);

        validator.assert_eq(String::from(actual), String::from(expected));
    }
}

fn concat(val: &str, _ignored_val: &str) -> String {
    format!("{}", val)
}
