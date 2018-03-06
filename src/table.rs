use std::iter::Iterator;
use std::vec::IntoIter;
use std::rc::Rc;
use std::cell::RefCell;
use test_case::TestCase;
use ansi_term::Colour::*;

pub struct Table<I, E> {
    values: IntoIter<(I, E)>,
    nb_test_failed: Rc<RefCell<usize>>,
    nb_of_tests: usize,
}

impl<I, E> Table<I, E> {
    pub fn new(values: Vec<(I, E)>) -> Table<I, E> {
        Table {
            nb_of_tests: values.len(),
            values: values.into_iter(),
            nb_test_failed: Rc::new(RefCell::new(0)),
        }
    }
}

impl<I, E> Drop for Table<I, E> {
    fn drop(&mut self) {
        let nb_test_failed = *self.nb_test_failed.borrow();
        if nb_test_failed > 0 {
            let header = "\n--------------------\n";
            let nb_test_passed = Green
                .bold()
                .paint(format!("{}", self.nb_of_tests - nb_test_failed));
            let nb_test_failed = Red.bold().paint(format!("{}", nb_test_failed));

            println!(
                "{}{} Passed {} Failed\n",
                header, nb_test_passed, nb_test_failed
            );
            panic!("Test Failed");
        }
    }
}

impl<I, E> Iterator for Table<I, E> {
    type Item = (TestCase, I, E);
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(value) = self.values.next() {
            Some((
                TestCase::new(Rc::clone(&self.nb_test_failed)),
                value.0,
                value.1,
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    pub fn given_ugly_names_when_validate_is_beautiful_then_table_panic() {
        let table = Table::new(vec![
            ("an ugly name", false),
            ("a beautiful name", true),
            ("an amazing name", true),
            ("worst name ever", false),
        ]);

        for (test_case, name, expected) in table {
            test_case
                .given(name)
                .when("validate is beautiful")
                .then(&format!("{}", expected))
                .assert_eq(expected, true);
        }
    }

    #[test]
    pub fn given_wonderful_names_when_validate_is_beautiful_then_table_dont_panic() {
        let table = Table::new(vec![("a beautiful name", true), ("an amazing name", true)]);

        for (test_case, name, expected) in table {
            test_case
                .given(name)
                .when("validate is beatifull")
                .then(&format!("{}", expected))
                .assert_eq(expected, true);
        }
    }
}
