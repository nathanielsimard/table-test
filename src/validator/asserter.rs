use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use formater::Formater;

pub struct Precision {
    pub tag: String,
    pub comment: String,
}

pub struct Asserter {
    precisions: Vec<Precision>,
    nb_failed: Rc<RefCell<usize>>,
    formater: Box<Formater>,
}

impl Asserter {
    pub fn new(
        precisions: Vec<Precision>,
        failed: Rc<RefCell<usize>>,
        formater: Box<Formater>,
    ) -> Asserter {
        Asserter {
            precisions: precisions,
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
            let mut output = self.formater.format_failed_test_header();

            for precision in &self.precisions {
                let line = self.formater
                    .format_one_line(&precision.tag, &precision.comment);
                output.push_str(&line);
            }
            output.push_str(&self.formater
                .format_diff(&format!("{:?}", expected), &format!("{:?}", actual)));

            output
        } else {
            let mut output = self.formater.format_passed_test_header();

            for precision in &self.precisions {
                let line = self.formater
                    .format_one_line(&precision.tag, &precision.comment);
                output.push_str(&line);
            }

            output
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

            let precisions = vec![
                Precision {
                    tag: "A Tag".to_string(),
                    comment: "A comment".to_string(),
                },
            ];

            let asserter = Asserter::new(precisions, failed.clone(), Box::new(formater.clone()));

            asserter.assert_eq(val_1, val_2);

            assert_eq!(format_failed_called, formater.format_failed_test_called());
            assert_eq!(
                format_failed_called,
                formater.format_diff_called_with(&format!("{:?}", val_1), &format!("{:?}", val_2))
            );
            assert_eq!(format_passed_called, formater.format_passed_test_called());
            assert!(
                formater
                    .format_one_line_called_with(&"A Tag".to_string(), &"A comment".to_string())
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

            let precisions = vec![
                Precision {
                    tag: "A Tag".to_string(),
                    comment: "A comment".to_string(),
                },
            ];

            let asserter = Asserter::new(precisions, failed.clone(), Box::new(formater.clone()));

            asserter.assert_ne(val_1, val_2);

            assert_eq!(format_failed_called, formater.format_failed_test_called());
            assert_eq!(
                format_failed_called,
                formater.format_diff_called_with(&format!("{:?}", val_2), &format!("{:?}", val_1))
            );
            assert_eq!(format_passed_called, formater.format_passed_test_called());
            assert!(
                formater
                    .format_one_line_called_with(&"A Tag".to_string(), &"A comment".to_string())
            );
            assert_eq!(nb_failed, *failed.borrow());
        }
    }
}
