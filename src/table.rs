use std::iter::Iterator;
use std::vec::IntoIter;
use std::rc::Rc;
use std::sync::Mutex;
use std::fmt::Debug;

use validator;
use validator::Validator;

pub struct Table<I, E> {
    values: IntoIter<(I, E)>,
    name: String,
    failed: Rc<Mutex<usize>>,
    number_of_tests: usize,
}

pub fn new<I, E>(name: &str, vec: Vec<(I, E)>) -> Table<I, E> {
    println!("\n");
    Table {
        number_of_tests: vec.len(),
        values: vec.into_iter(),
        failed: Rc::new(Mutex::new(0)),
        name: String::from(name),
    }
}

impl<I, E> Drop for Table<I, E> {
    fn drop(&mut self) {
        let failed = *self.failed.lock().unwrap();
        if failed > 0 {
            println!("{} passed {} failed\n", self.number_of_tests - failed, failed);
            panic!("Test Failed");
        }
    }
}

impl<I: Debug, E: Debug> Iterator for Table<I, E> {
    type Item = (Validator, I, E);
    fn next(&mut self) -> Option<Self::Item> {
        let items = self.values.next();

        match items {
            Some(value) => {
                let inputs = format!("{:?}", value.0);
                let result = (
                    validator::new(self.name.clone(), inputs, Rc::clone(&self.failed)),
                    value.0,
                    value.1,
                );

                Some(result)
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    pub fn given_ugly_names_when_validate_is_beautiful_then_table_panic() {
        let table = new(
            "name is beatiful",
            vec![
                ("an ugly name", false),
                ("a beautiful name", true),
                ("an amazing name", true),
                ("worst name ever", false),
            ],
        );

        for (validator, _, expected) in table {
            validator.assert_eq(expected, true);
        }
    }

    #[test]
    pub fn given_wonderful_names_when_validate_is_beautiful_then_table_dont_panic() {
        let table = new(
            "name is beatiful",
            vec![("a beautiful name", true), ("an amazing name", true)],
        );

        for (validator, _, expected) in table {
            validator.assert_eq(&expected, &true);
        }
    }
}
