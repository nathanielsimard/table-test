use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use formater::Formater;

pub struct Asserter {
    given: String,
    when: String,
    then: String,
    nb_failed: Rc<RefCell<usize>>,
    formater: Box<Formater>,
}

impl Asserter {
    pub fn new(
        given: String,
        when: String,
        then: String,
        failed: Rc<RefCell<usize>>,
        formater: Box<Formater>,
    ) -> Asserter {
        Asserter {
            given: given,
            when: when,
            then: then,
            nb_failed: failed,
            formater: formater,
        }
    }

    pub fn assert_eq<T: PartialEq + Debug>(self, expected: T, actual: T) {
        let passed = expected == actual;
        print!("\n{}", self.format_test(passed, expected, actual))
    }

    pub fn assert_ne<T: PartialEq + Debug>(self, expected: T, actual: T) {
        let passed = expected != actual;
        print!("\n{}", self.format_test(passed, expected, actual))
    }

    fn format_test<T: PartialEq + Debug>(&self, passed: bool, expected: T, actual: T) -> String {
        if !passed {
            self.increment_failed_counter();
            self.formater.format_failed_test(
                &self.given,
                &self.when,
                &self.then,
                &String::from(format!("{:?}", expected)),
                &String::from(format!("{:?}", actual)),
            )
        } else {
            self.formater
                .format_passed_test(&self.given, &self.when, &self.then)
        }
    }

    fn increment_failed_counter(&self) {
        let mut nb_failed = self.nb_failed.borrow_mut();
        *nb_failed += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use formater::mock::Mock;

    #[test]
    fn test_assert_eq() {
        let table = vec![
            ("something", "something else", 1, true, false),
            ("something", "something", 0, false, true),
        ];

        for (val_1, val_2, nb_failed, format_failed_called, format_passed_called) in table {
            let formater = Mock::new();
            let failed = Rc::new(RefCell::new(0));

            let given = String::from("given");
            let when = String::from("when");
            let then = String::from("then");

            let asserter = Asserter::new(
                given.clone(),
                when.clone(),
                then.clone(),
                failed.clone(),
                Box::new(formater.clone()),
            );

            asserter.assert_eq(val_1, val_2);

            assert_eq!(
                format_failed_called,
                formater.format_failed_test_called_with(
                    &given,
                    &when,
                    &then,
                    &format!("{:?}", val_1),
                    &format!("{:?}", val_2),
                )
            );
            assert_eq!(
                format_passed_called,
                formater.format_passed_test_called_with(&given, &when, &then)
            );
            assert_eq!(nb_failed, *failed.borrow());
        }
    }

    #[test]
    fn test_assert_ne() {
        let table = vec![
            ("something", "something else", 0, false, true),
            ("something", "something", 1, true, false),
        ];

        for (val_1, val_2, nb_failed, format_failed_called, format_passed_called) in table {
            let formater = Mock::new();
            let failed = Rc::new(RefCell::new(0));

            let given = String::from("given");
            let when = String::from("when");
            let then = String::from("then");

            let asserter = Asserter::new(
                given.clone(),
                when.clone(),
                then.clone(),
                failed.clone(),
                Box::new(formater.clone()),
            );

            asserter.assert_ne(val_1, val_2);

            assert_eq!(
                format_failed_called,
                formater.format_failed_test_called_with(
                    &given,
                    &when,
                    &then,
                    &format!("{:?}", val_2),
                    &format!("{:?}", val_1),
                )
            );
            assert_eq!(
                format_passed_called,
                formater.format_passed_test_called_with(&given, &when, &then)
            );
            assert_eq!(nb_failed, *failed.borrow());
        }
    }
}
