extern crate ansi_term;
extern crate difference;

#[macro_use]
pub mod macros;
pub mod table;

mod formater;
mod validator;

fn add2(val_1: i64, val_2: i64) -> i64 {
    val_1 * val_2 // Oups used * instead of +
}

pub fn tmp() {
    let table = vec![
        ((1, 2), 3),
        ((2, 5), 7),
        ((0, 0), 0),
        ((0, 1), 1),
        ((2, 2), 4),
    ];

    for (validator, (input_1, input_2), expected) in table_test!(table) {
        let actual = add2(input_1, input_2);

        validator
            .given("")
            .when("")
            .then("")
            .assert_eq(expected, actual);
    }
}
