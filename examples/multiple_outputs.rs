#[macro_use]
extern crate table_test;

fn main() {
    let table = table_test!(
        split,
        vec![
            ("John Smith", ("John", "Smith")),
            ("Ian Taylor", ("Ian", "Taylor")),
            ("Justin Williams", ("Justin", "Williams")),
        ]
    );

    for (validator, input, (expected_1, expected_2)) in table {
        let mut actuals = input.split_whitespace();

        let name = actuals.next();
        validator
            .when("split name")
            .assert_eq(Some(expected_1), name);

        let surname = actuals.next();
        validator
            .when("split surname")
            .assert_eq(Some(expected_2), surname);

        let none = actuals.next();
        validator
            .when("there is no more split")
            .assert_eq(None, none);
    }
}
