use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use formater;

#[derive(Clone)]
pub struct Validator {
    inputs: String,
    given: String,
    when: String,
    then: String,
    nb_failed: Rc<RefCell<usize>>,
}

#[derive(Clone)]
pub struct Given {
    inputs: String,
    failed: Rc<RefCell<usize>>,
    value: String,
}

pub struct When {
    given: Given,
    value: String,
}

pub struct Then {
    when: When,
}

impl Given {
    pub fn new(inputs: String, failed: Rc<RefCell<usize>>) -> Given {
        Given {
            inputs: inputs,
            failed: failed,
            value: String::new(),
        }
    }
    pub fn given(mut self, given: &str) -> When {
        self.value = given.to_string();
        When {
            given: self,
            value: String::new(),
        }
    }
}

impl When {
    pub fn when(mut self, when: &str) -> Then {
        self.value = when.to_string();
        Then { when: self }
    }
}

impl Then {
    pub fn then(self, then: &str) -> Validator {
        Validator::new(
            self.when.given.value,
            self.when.value,
            then.to_string(),
            self.when.given.inputs,
            self.when.given.failed,
        )
    }
}
impl Validator {
    pub fn new(
        given: String,
        when: String,
        then: String,
        inputs: String,
        failed: Rc<RefCell<usize>>,
    ) -> Validator {
        Validator {
            inputs: inputs.clone(),
            given: given,
            when: when,
            then: then,
            nb_failed: failed,
        }
    }

    pub fn assert_eq<T: PartialEq + Debug>(self, expected: T, actual: T) {
        if expected != actual {
            self.increment_failed_counter();
            print!(
                "\n{}",
                formater::format_failed_test(
                    &self.given,
                    &self.when,
                    &self.then,
                    &String::from(format!("{:?}", expected)),
                    &String::from(format!("{:?}", actual)),
                )
            );
        } else {
            print!(
                "\n{}",
                formater::format_passed_test(&self.given, &self.when, &self.then)
            );
        }
    }

    fn increment_failed_counter(&self) {
        let mut nb_failed = self.nb_failed.borrow_mut();
        *nb_failed += 1;
    }
}
