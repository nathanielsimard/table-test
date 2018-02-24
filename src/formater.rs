use ansi_term::Colour::*;
use ansi_term::Style;
use difference::{Changeset, Difference};

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
    let actual = format!("{}\n", diff(expected, actual));

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

fn diff(expected: &String, actual: &String) -> String {
    let Changeset { diffs, .. } = Changeset::new(expected, actual, "");
    let mut actual = String::new();
    let mut expected = String::new();

    for diff in &diffs {
        match *diff {
            Difference::Same(ref same) => {
                actual.push_str(&format!("{}", Red.paint(format!("{}", same))));
                expected.push_str(&format!("{}", Green.paint(format!("{}", same))));
            }
            Difference::Rem(ref rem) => {
                expected.push_str(&format!(
                    "{}",
                    Style::new().on(Green).fg(White).paint(format!("{}", rem))
                ));
            }
            Difference::Add(ref add) => {
                actual.push_str(&format!(
                    "{}",
                    Style::new().on(Red).fg(White).paint(format!("{}", add))
                ));
            }
        }
    }
    String::from(format!("  {}\n  {}", actual, expected))
}
