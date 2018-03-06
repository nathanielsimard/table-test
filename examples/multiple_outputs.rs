#[macro_use]
extern crate table_test;

fn main() {
    let table = vec![
        ("John Smith", ("John", "Smith")),
        ("Ian Taylor", ("Ian", "Taylor")),
        ("Justin Williams", ("Justin", "Williams")),
    ];

    for (test_case, input, (expected_1, expected_2)) in table_test!(table) {
        let mut actuals = input.split_whitespace();

        let name = actuals.next();
        let test_case = test_case
            .given("a name and surname")
            .when("split whitespace")
            .then("first next returns the name")
            .assert_eq(Some(expected_1), name);

        let surname = actuals.next();
        let test_case = test_case
            .then("second next returns the surname")
            .assert_eq(Some(expected_2), surname);

        let none = actuals.next();
        test_case
            .then("third next returns None")
            .assert_eq(None, none);
    }
}
