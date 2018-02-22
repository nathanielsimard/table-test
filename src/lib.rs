extern crate ansi_term;

use std::iter::Iterator;
use std::vec::IntoIter;
use std::fmt::Debug;
use ansi_term::Colour::*;
use ansi_term::Style;
use std::sync::Mutex;

struct Table<T> {
    values: Mutex<IntoIter<T>>,
    validator: Validator,
}

struct Validator {
    name: String,
    output: Mutex<Vec<String>>,
    number_failed: Mutex<usize>,
}
fn new<T>(name: &str, vec: Vec<T>) -> Table<T> {
    Table {
        values: Mutex::new(vec.into_iter()),
        validator: Validator {
            name: String::from(name),
            output: Mutex::new(vec![]),
            number_failed: Mutex::new(0),
        },
    }
}

impl Validator {
    pub fn assert_eq<T: PartialEq + Debug>(&self, expected: T, actual: T, comment: &str) {
        let mut output = self.output.lock().unwrap();
        if expected != actual {
            let mut data = self.number_failed.lock().unwrap();
            *data += 1;
            output.push(String::from(format!(
                "[{}]\n  {} {}\n  {} {}\n  {} {}\n  -  {}:\n\t{:?}\n  -  {}:\n\t{:?}",
                Style::new()
                    .bold()
                    .paint(format!("{}", Red.paint("Failed"))),
                Cyan.paint("Given"),
                comment,
                Cyan.paint("When"),
                self.name,
                Cyan.paint("Then"),
                comment,
                Cyan.paint("Expected"),
                expected,
                Cyan.paint("Actual"),
                actual
            )));
        } else {
            output.push(String::from(format!(
                "[{}]\n  {} {}\n  {} {}\n  {} {}",
                Style::new()
                    .bold()
                    .paint(format!("{}", Green.paint("Passed"))),
                Cyan.paint("Given"),
                comment,
                Cyan.paint("When"),
                comment,
                Cyan.paint("Then"),
                comment,
            )));
        }
    }

    fn format(&self) {
        let data = self.number_failed.lock().unwrap();
        let output = self.output.lock().unwrap();
        println!("\n----------------------------------------");
        println!(
            "Result: {} failed {} passed\n",
            Red.paint(format!("{}", data)),
            Green.paint(format!("{}", (output.len() - *data))),
        );
        for output in output.iter() {
            println!("{}\n", output);
        }
        println!("\n----------------------------------------");
        if *data > 0 {
            panic!("Test Failed");
        }
    }
}

impl<T> Drop for Table<T> {
    fn drop(&mut self) {
        self.validator.format();
    }
}

impl<'a, T> Iterator for &'a Table<T> {
    type Item = (&'a Validator, T);
    fn next(&mut self) -> Option<Self::Item> {
        let mut values = self.values.lock().unwrap();
        let items = values.next();
        match items {
            Some(value) => {
                let result = (&self.validator, value);
                return Some(result);
            }
            None => None,
        }
    }
}

fn name_is_beatiful(_name: &str) -> bool {
    true
}

pub fn name_is_beatiful_test() {
    let table = new(
        "name is beatiful",
        vec![
            ("an ugly name", true),
            ("a beautiful name", true),
            ("an amazing name", false),
            ("worst name ever", false),
        ],
    );

    for (validator, (name, expected)) in &table {
        let actual = name_is_beatiful(name);

        validator.assert_eq(&expected, &actual, "allo");
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        name_is_beatiful_test();
    }
}
