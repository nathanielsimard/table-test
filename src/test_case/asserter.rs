use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use formater::Formater;

pub struct Asserter {
    output: String,
    nb_failed: Rc<RefCell<usize>>,
    formater: Box<Formater>,
    failed: bool,
}

impl Drop for Asserter {
    fn drop(&mut self) {
        self.format_test();
        print!("\n{}", self.output);
    }
}

impl Asserter {
    pub fn new(failed: Rc<RefCell<usize>>, formater: Box<Formater>) -> Asserter {
        Asserter {
            output: String::new(),
            nb_failed: failed,
            formater: formater,
            failed: false,
        }
    }

    pub fn add_comment(&mut self, tag: String, value: String) {
        let formated_comment = self.formater.format_one_line(&tag, &value);
        self.output.push_str(&formated_comment);
    }

    pub fn assert_eq<T: PartialEq + Debug>(&mut self, expected: T, actual: T) {
        if expected != actual {
            self.failed = true;
            self.format_diff(expected, actual);
        }
    }

    pub fn assert_ne<T: PartialEq + Debug>(&mut self, expected: T, actual: T) {
        if expected == actual {
            self.failed = true;
            self.format_diff(expected, actual);
        }
    }

    fn format_diff<T: PartialEq + Debug>(&mut self, expected: T, actual: T) {
        let diff = self.formater
            .format_diff(&format!("{:?}", expected), &format!("{:?}", actual));
        self.output.push_str(&diff);
    }

    fn format_test(&mut self) {
        if self.failed {
            self.increment_failed_counter();
            let header = self.formater.format_failed_test_header();
            self.output = format!("{}{}", header, self.output);
        } else {
            let header = self.formater.format_passed_test_header();
            self.output = format!("{}{}", header, self.output);
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
            let mut asserter = Asserter::new(failed.clone(), Box::new(formater.clone()));

            asserter.assert_eq(val_1, val_2);
            asserter.format_test();

            assert_eq!(format_failed_called, formater.format_failed_test_called());
            assert_eq!(
                format_failed_called,
                formater.format_diff_called_with(&format!("{:?}", val_1), &format!("{:?}", val_2))
            );
            assert_eq!(format_passed_called, formater.format_passed_test_called());
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
            let mut asserter = Asserter::new(failed.clone(), Box::new(formater.clone()));

            asserter.assert_ne(val_1, val_2);
            asserter.format_test();

            assert_eq!(format_failed_called, formater.format_failed_test_called());
            assert_eq!(
                format_failed_called,
                formater.format_diff_called_with(&format!("{:?}", val_2), &format!("{:?}", val_1))
            );
            assert_eq!(format_passed_called, formater.format_passed_test_called());
            assert_eq!(nb_failed, *failed.borrow());
        }
    }

    #[test]
    fn given_a_tag_and_a_comment_when_add_comment_then_formater_has_been_called() {
        let formater = Mock::new();
        let failed = Rc::new(RefCell::new(0));
        let mut asserter = Asserter::new(failed.clone(), Box::new(formater.clone()));
        let tag = String::from("A Tag");
        let comment = String::from("A comment");

        asserter.add_comment(tag.clone(), comment.clone());

        assert!(formater.format_one_line_called_with(&tag, &comment));
    }

    #[test]
    fn given_a_tag_and_a_comment_when_add_comment_then_output_has_changed() {
        let mut formater = Mock::new();
        formater.format_one_line_return = "A Tag A comment".to_string();
        let failed = Rc::new(RefCell::new(0));
        let mut asserter = Asserter::new(failed.clone(), Box::new(formater.clone()));
        let tag = String::from("A Tag");
        let comment = String::from("A comment");
        let previous_ouput = asserter.output.clone();

        asserter.add_comment(tag.clone(), comment.clone());

        assert_ne!(asserter.output, previous_ouput);
    }
}
