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

    for (test_case, inputs, output) in table_test!(table) {
        let actual = add(inputs.0, inputs.1);

        test_case
            .description("test the add function")
            .custom("Inputs", &format!("{:?}", inputs))
            .custom("Output", &format!("{:?}", output))
            .assert_eq(actual, output);
    }
}
