use formater::Formater;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Mock {
    format_passed_test_called: Rc<RefCell<FormatPassedTestInputs>>,
    format_failed_test_called: Rc<RefCell<FormatFailedTestInputs>>,
}

#[derive(PartialEq, Debug)]
struct FormatPassedTestInputs {
    given: String,
    when: String,
    then: String,
}

#[derive(PartialEq, Debug)]
struct FormatFailedTestInputs {
    given: String,
    when: String,
    then: String,
    expected: String,
    actual: String,
}

fn new_empty_passed_inputs() -> FormatPassedTestInputs {
    FormatPassedTestInputs {
        given: String::from(""),
        when: String::from(""),
        then: String::from(""),
    }
}

fn new_empty_failed_inputs() -> FormatFailedTestInputs {
    FormatFailedTestInputs {
        given: String::from(""),
        when: String::from(""),
        then: String::from(""),
        expected: String::from(""),
        actual: String::from(""),
    }
}

impl Formater for Mock {
    fn format_passed_test(&self, given: &String, when: &String, then: &String) -> String {
        *self.format_passed_test_called.borrow_mut() = FormatPassedTestInputs {
            given: given.clone(),
            when: when.clone(),
            then: then.clone(),
        };
        String::new()
    }
    fn format_failed_test(
        &self,
        given: &String,
        when: &String,
        then: &String,
        expected: &String,
        actual: &String,
    ) -> String {
        *self.format_failed_test_called.borrow_mut() = FormatFailedTestInputs {
            given: given.clone(),
            when: when.clone(),
            then: then.clone(),
            expected: expected.clone(),
            actual: actual.clone(),
        };
        String::new()
    }
}

impl Mock {
    pub fn new() -> Mock {
        Mock {
            format_passed_test_called: Rc::new(RefCell::new(new_empty_passed_inputs())),
            format_failed_test_called: Rc::new(RefCell::new(new_empty_failed_inputs())),
        }
    }
    pub fn format_passed_test_called_with(
        &self,
        given: &String,
        when: &String,
        then: &String,
    ) -> bool {
        let format_inputs = self.format_passed_test_called.borrow_mut();
        let format = FormatPassedTestInputs {
            given: given.clone(),
            when: when.clone(),
            then: then.clone(),
        };
        *format_inputs == format
    }

    pub fn format_failed_test_called_with(
        &self,
        given: &String,
        when: &String,
        then: &String,
        expected: &String,
        actual: &String,
    ) -> bool {
        let format_inputs = self.format_failed_test_called.borrow();
        let format = FormatFailedTestInputs {
            given: given.clone(),
            when: when.clone(),
            then: then.clone(),
            expected: expected.clone(),
            actual: actual.clone(),
        };
        *format_inputs == format
    }
}
