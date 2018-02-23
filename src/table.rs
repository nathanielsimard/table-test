use std::iter::Iterator;
use std::vec::IntoIter;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use validator::Validator;
use ansi_term::Colour::*;

pub struct Table<I, E> {
    values: IntoIter<(I, E)>,
    name: String,
    nb_failed: Rc<RefCell<usize>>,
    number_of_tests: usize,
}

impl<I, E> Table<I, E> {
    pub fn new(name: &str, vec: Vec<(I, E)>) -> Table<I, E> {
        Table {
            number_of_tests: vec.len(),
            values: vec.into_iter(),
            nb_failed: Rc::new(RefCell::new(0)),
            name: String::from(name),
        }
    }
}

impl<I, E> Drop for Table<I, E> {
    fn drop(&mut self) {
        let nb_failed = *self.nb_failed.borrow_mut();
        if nb_failed > 0 {
            println!("\n--------------------");
            let nb_passed = Green.bold().paint(format!("{}", self.number_of_tests - nb_failed));
            let nb_failed = Red.bold().paint(format!("{}", nb_failed));
            println!("{} Passed {} Failed\n", nb_passed, nb_failed);
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
                    Validator::new(self.name.clone(), inputs, Rc::clone(&self.nb_failed)),
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
        let table = Table::new(
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
        let table = Table::new(
            "name is beatiful",
            vec![("a beautiful name", true), ("an amazing name", true)],
        );

        for (validator, _, expected) in table {
            validator.assert_eq(&expected, &true);
        }
    }
}
