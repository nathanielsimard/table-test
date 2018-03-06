#[macro_use]
extern crate table_test;

fn add(val_1: i64, val_2: i64) -> i64 {
    val_1 * val_2 // Oups used * instead of +
}
fn main() {
    let table = vec![
        ((1, 2), 3),
        ((2, 5), 7),
        ((0, 0), 0),
        ((0, 1), 1),
        ((2, 2), 4),
    ];

    for (test_case, (input_1, input_2), expected) in table_test!(table) {
        let actual = add(input_1, input_2);

        test_case
            .given(&format!("{}, {}", input_1, input_2))
            .when("add")
            .then(&format!("it should be {}", expected))
            .assert_eq(expected, actual);
    }
}
