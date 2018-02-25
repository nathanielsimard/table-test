use std::rc::Rc;
use std::cell::RefCell;
use validator::test_structure::{Given, When};

#[derive(Clone)]
pub struct Validator {
    failed: Rc<RefCell<usize>>,
}

impl Validator {
    pub fn new(failed: Rc<RefCell<usize>>) -> Validator {
        Validator { failed: failed }
    }

    pub fn given(self, given: &str) -> When {
        When::new(Given::new(self.failed, given.to_string()))
    }
}
