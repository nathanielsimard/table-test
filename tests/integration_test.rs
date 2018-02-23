#[macro_use]
extern crate table_test;

fn correct_add(val_1: i64, val_2: i64, val_3: i64) -> i64 {
    val_1 + val_2 + val_3
}

fn incorrect_add(val_1: i64, val_2: i64, _val_3: i64) -> i64 {
    val_1 + val_2
}

#[test]
fn given_correct_add_function_when_use_table_test_then_no_panic() {
    let table = table_test!(
        "correct_add",
        [
            ((1, 2, 0), 3),
            ((1, 5, 1), 7),
            ((0, 0, 0), 0),
            ((0, 1, -6), -5)
        ]
    );

    for (validator, (input_1, input_2, input_3), expected) in table {
        let actual = correct_add(input_1, input_2, input_3);

        validator.assert_eq(actual, expected);
    }
}

#[test]
#[should_panic]
fn given_incorrect_add_function_when_use_table_test_then_panic() {
    let table = table_test!(
        "incorrect_add",
        [
            ((1, 2, 0), 3),
            ((1, 5, 1), 7),
            ((0, 0, 0), 0),
            ((0, 1, -6), -5)
        ]
    );

    for (validator, (input_1, input_2, input_3), expected) in table {
        let actual = incorrect_add(input_1, input_2, input_3);

        validator.assert_eq(actual, expected);
    }
}
