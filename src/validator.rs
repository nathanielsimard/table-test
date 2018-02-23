use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use formater;

pub struct Validator {
    inputs: String,
    name: String,
    nb_failed: Rc<RefCell<usize>>,
}

impl Validator {
    pub fn new(name: String, inputs: String, failed: Rc<RefCell<usize>>) -> Validator {
        Validator {
            name: name,
            inputs: inputs,
            nb_failed: failed,
        }
    }
    pub fn assert_eq<T: PartialEq + Debug>(&self, expected: T, actual: T) {
        if expected != actual {
            self.increment_failed_counter();
            let output = formater::format_failed_test(
                &self.inputs,
                &self.name,
                &String::from(format!("{:?}", actual)),
                &String::from(format!("{:?}", expected)),
            );

            print!("\n{}", output);
        } else {
            let output = formater::format_passed_test(
                &self.inputs,
                &self.name,
                &String::from(format!("{:?}", expected)),
            );
            print!("\n{}", output);
        }
    }

    fn increment_failed_counter(&self) {
        let mut nb_failed = self.nb_failed.borrow_mut();
        *nb_failed += 1;
    }
}
