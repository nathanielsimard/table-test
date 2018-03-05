use formater::Formater;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct Mock {
    format_one_line_called: Rc<RefCell<FormatOneLineInputs>>,
    format_diff_called: Rc<RefCell<FormatDiffInputs>>,
    format_passed_test_called: Rc<RefCell<bool>>,
    format_failed_test_called: Rc<RefCell<bool>>,
}

#[derive(PartialEq, Debug)]
struct FormatOneLineInputs {
    tag: String,
    comment: String,
}

#[derive(PartialEq, Debug)]
struct FormatDiffInputs {
    expected: String,
    actual: String,
}

fn new_empty_format_one_line_inputs() -> FormatOneLineInputs {
    FormatOneLineInputs {
        tag: String::from(""),
        comment: String::from(""),
    }
}

fn new_empty_format_diff_inputs() -> FormatDiffInputs {
    FormatDiffInputs {
        expected: String::from(""),
        actual: String::from(""),
    }
}

impl Formater for Mock {
    fn format_one_line(&self, tag: &String, comment: &String) -> String {
        *self.format_one_line_called.borrow_mut() = FormatOneLineInputs {
            tag: tag.clone(),
            comment: comment.clone(),
        };
        String::new()
    }
    fn format_passed_test_header(&self) -> String {
        *self.format_passed_test_called.borrow_mut() = true;
        String::new()
    }
    fn format_failed_test_header(&self) -> String {
        *self.format_failed_test_called.borrow_mut() = true;
        String::new()
    }
    fn format_diff(&self, expected: &String, actual: &String) -> String {
        *self.format_diff_called.borrow_mut() = FormatDiffInputs {
            expected: expected.clone(),
            actual: actual.clone(),
        };
        String::new()
    }
}

impl Mock {
    pub fn new() -> Mock {
        Mock {
            format_diff_called: Rc::new(RefCell::new(new_empty_format_diff_inputs())),
            format_one_line_called: Rc::new(RefCell::new(new_empty_format_one_line_inputs())),
            format_passed_test_called: Rc::new(RefCell::new(false)),
            format_failed_test_called: Rc::new(RefCell::new(false)),
        }
    }
    pub fn format_passed_test_called(&self) -> bool {
        *self.format_passed_test_called.borrow()
    }

    pub fn format_diff_called_with(&self, expected: &String, actual: &String) -> bool {
        let format_inputs = self.format_diff_called.borrow();
        println!("Expected :{:?}\n", format_inputs);
        let format = FormatDiffInputs {
            expected: expected.clone(),
            actual: actual.clone(),
        };
        println!("Actual :{:?}\n", format);
        *format_inputs == format
    }

    pub fn format_failed_test_called(&self) -> bool {
        *self.format_failed_test_called.borrow()
    }

    pub fn format_one_line_called_with(&self, tag: &String, comment: &String) -> bool {
        let format_inputs = self.format_one_line_called.borrow();
        let format = FormatOneLineInputs {
            tag: tag.clone(),
            comment: comment.clone(),
        };
        *format_inputs == format
    }
}
