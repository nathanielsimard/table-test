#[macro_use]
extern crate table_test;

fn main() {
    let table = vec![
        ("John Smith", ("John", "Smith")),
        ("Ian Taylor", ("Ian", "Taylor")),
        ("Justin Williams", ("Justin", "Williams")),
    ];

    for (validator, input, (expected_1, expected_2)) in table_test!(table) {
        let mut actuals = input.split_whitespace();

        let name = actuals.next();
        validator
            .clone()
            .given("a name and surname")
            .when("split name")
            .then("return name")
            .assert_eq(Some(expected_1), name);

        let surname = actuals.next();
        validator
            .clone()
            .given("a name and surname")
            .when("split surname")
            .then("return surname")
            .assert_eq(Some(expected_2), surname);

        let none = actuals.next();
        validator
            .given("a name and a surname")
            .when("there is no more split")
            .then("None")
            .assert_eq(None, none);
    }
}
