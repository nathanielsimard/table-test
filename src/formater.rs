use ansi_term::Colour::*;
use difference::Changeset;

pub fn format_failed_test(
    inputs: &String,
    test_name: &String,
    expected: &String,
    actual: &String,
) -> String {
    let status = format!(
        "{}{}{}\n",
        "[",
        Red.bold().paint(String::from("Failed")),
        "]"
    );
    let given_when_then = format_core(inputs, test_name, expected);
    let actual = format!(
        "{}{}",
        Cyan.paint("Receive:\n\t"),
        Changeset::new(actual, expected, "\n\t")
    );

    return String::from(format!("{}{}{}", status, given_when_then, actual));
}

pub fn format_passed_test(inputs: &String, test_name: &String, expected: &String) -> String {
    let status = format!(
        "{}{}{}\n",
        "[",
        Green.bold().paint(String::from("Passed")),
        "]"
    );
    let given_when_then = format_core(inputs, test_name, expected);
    return String::from(format!("{}{}", status, given_when_then));
}

fn format_core(inputs: &String, test_name: &String, expected: &String) -> String {
    let given = format!("{}{}\n", Cyan.paint("Given "), inputs);
    let when = format!("{}{}\n", Cyan.paint("When "), test_name);
    let it_should = format!("{}{}\n", Cyan.paint("It should return "), expected);
    return String::from(format!("{}{}{}", given, when, it_should));
}
