use ansi_term::Colour::*;
use ansi_term::Style;
use difference::{Changeset, Difference};
use formater::Formater;

pub struct ColorfulFormater;

impl Formater for ColorfulFormater {
    fn format_failed_test(
        &self,
        given: &String,
        when: &String,
        then: &String,
        expected: &String,
        actual: &String,
    ) -> String {
        let status = format!(
            "{}{}{}\n",
            "[",
            Red.bold().paint(String::from("Failed")),
            "]"
        );
        let given_when_then = format_core(given, when, then);
        let actual = format!("{}\n", diff(expected, actual));

        return String::from(format!("{}{}{}", status, given_when_then, actual));
    }

    fn format_passed_test(&self, given: &String, when: &String, then: &String) -> String {
        let status = format!(
            "{}{}{}\n",
            "[",
            Green.bold().paint(String::from("Passed")),
            "]"
        );
        let given_when_then = format_core(given, when, then);
        return String::from(format!("{}{}", status, given_when_then));
    }
}

impl ColorfulFormater {
    pub fn new() -> ColorfulFormater {
        ColorfulFormater {}
    }
}

fn format_core(given: &String, when: &String, then: &String) -> String {
    let given = format!("{}{}\n", Cyan.paint("Given "), given);
    let when = format!("{}{}\n", Cyan.paint("When "), when);
    let then = format!("{}{}\n", Cyan.paint("Then "), then);

    return String::from(format!("{}{}{}", given, when, then));
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
                    Style::new().on(Green).paint(format!("{}", rem))
                ));
            }
            Difference::Add(ref add) => {
                actual.push_str(&format!(
                    "{}",
                    Style::new().on(Red).paint(format!("{}", add))
                ));
            }
        }
    }
    String::from(format!("  {}\n  {}", actual, expected))
}
