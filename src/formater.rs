use ansi_term::Colour::*;

pub fn format_failed_test(
    inputs: &String,
    test_name: &String,
    assert_method: &String,
    expected: &String,
    actual: &String,
) -> String {
    let status = format!("{}{}{}\n", "[", Red.paint(String::from("Failed")), "]");
    let given_when_then = format_given_when_then(inputs, test_name, assert_method);
    let expected = format!("{}{}\n", Cyan.paint("  Expected:\n    "), expected);
    let actual = format!("{}{}\n", Cyan.paint("  Actual:\n    "), actual);

    return String::from(format!(
        "{}{}{}{}",
        status, given_when_then, expected, actual
    ));
}

pub fn format_passed_test(
    inputs: &String,
    test_name: &String,
    assert_method: &String,
) -> String {
    let status = format!("{}{}{}\n", "[", Green.paint(String::from("Passed")), "]");
    let given_when_then = format_given_when_then(inputs, test_name, assert_method);
    return String::from(format!("{}{}", status, given_when_then));
}

fn format_given_when_then(inputs: &String, test_name: &String, assert_method: &String) -> String {
    let given = format!("{}{}\n", Cyan.paint("  Given:\n    "), inputs);
    let when = format!("{}{}\n", Cyan.paint("  When:\n    "), test_name);
    let then = format!("{}{}\n", Cyan.paint("  Then:\n    "), assert_method);
    return String::from(format!("{}{}{}", given, when, then));
}
