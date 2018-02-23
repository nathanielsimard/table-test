use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Mutex;
use formater;

pub struct Validator {
    inputs: String,
    name: String,
    failed: Rc<Mutex<usize>>,
}

pub fn new(name: String, inputs: String, failed: Rc<Mutex<usize>>) -> Validator {
    Validator {
        name: name,
        inputs: inputs,
        failed: failed,
    }
}

impl Validator {
    pub fn assert_eq<T: PartialEq + Debug>(&self, expected: T, actual: T) {
        if expected != actual {
            self.increment_failed_counter();
            let output = formater::format_failed_test(
                &self.inputs,
                &self.name,
                &String::from("assert equal"),
                &String::from(format!("{:?}", expected)),
                &String::from(format!("{:?}", actual)),
            );

            println!("{}", output);
        } else {
            let output = formater::format_passed_test(
                &self.inputs,
                &self.name,
                &String::from("assert equal"),
            );
            println!("{}", output);
        }
    }

    fn increment_failed_counter(&self) {
        let mut failed = self.failed.lock().unwrap();
        *failed += 1;
    }
}
