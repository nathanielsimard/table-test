use ansi_term::Colour::*;
use ansi_term::Style;
use difference::{Changeset, Difference};
use formater::Formater;

pub struct ColorfulFormater;

impl Formater for ColorfulFormater {
    fn format_one_line(&self, tag: &String, comment: &String) -> String {
        String::from(format!("{} {}\n", Cyan.paint(tag.clone()), comment))
    }

    fn format_diff(&self, expected: &String, actual: &String) -> String {
        diff(expected, actual)
    }

    fn format_failed_test_header(&self) -> String {
        String::from(format!(
            "{}{}{}\n",
            "[",
            Red.bold().paint(String::from("Failed")),
            "]"
        ))
    }

    fn format_passed_test_header(&self) -> String {
        String::from(format!(
            "{}{}{}\n",
            "[",
            Green.bold().paint(String::from("Passed")),
            "]"
        ))
    }
}

impl ColorfulFormater {
    pub fn new() -> ColorfulFormater {
        ColorfulFormater {}
    }
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
    String::from(format!("  {}\n  {}\n", expected, actual))
}
