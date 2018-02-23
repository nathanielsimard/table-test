use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Mutex;

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
    pub fn assert_eq<T: PartialEq + Debug>(&mut self, expected: T, actual: T) {
        if expected != actual {
            let mut failed = self.failed.lock().unwrap();
            *failed += 1;
        }
    }
}
