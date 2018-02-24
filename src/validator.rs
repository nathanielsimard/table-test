use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use formater;

#[derive(Clone)]
pub struct Validator {
    inputs: String,
    given: String,
    when: String,
    then: Option<String>,
    nb_failed: Rc<RefCell<usize>>,
}

impl Validator {
    pub fn new(name: String, inputs: String, failed: Rc<RefCell<usize>>) -> Validator {
        Validator {
            inputs: inputs.clone(),
            when: name,
            given: inputs,
            nb_failed: failed,
            then: None,
        }
    }

    pub fn given(mut self, given: &str) -> Validator {
        self.given = given.to_string();
        self
    }

    pub fn when(mut self, when: &str) -> Validator {
        self.when = when.to_string();
        self
    }

    pub fn then(mut self, then: &str) -> Validator {
        self.then = Some(then.to_string());
        self
    }

    pub fn assert_eq<T: PartialEq + Debug>(self, expected: T, actual: T) {
        if expected != actual {
            self.increment_failed_counter();
            match self.then {
                Some(then) => {
                    print!(
                        "\n{}",
                        formater::format_failed_test(
                            &self.given,
                            &self.when,
                            &then,
                            &String::from(format!("{:?}", expected)),
                            &String::from(format!("{:?}", actual)),
                        )
                    );
                }
                None => {
                    print!(
                        "\n{}",
                        formater::format_failed_test(
                            &self.given,
                            &self.when,
                            &String::from(format!("{:?}", expected)),
                            &String::from(format!("{:?}", expected)),
                            &String::from(format!("{:?}", actual)),
                        )
                    );
                }
            }
        } else {
            match self.then {
                Some(then) => {
                    print!(
                        "\n{}",
                        formater::format_passed_test(&self.given, &self.when, &then,)
                    );
                }
                None => {
                    print!(
                        "\n{}",
                        formater::format_passed_test(
                            &self.given,
                            &self.when,
                            &String::from(format!("{:?}", expected)),
                        )
                    );
                }
            }
        }
    }

    fn increment_failed_counter(&self) {
        let mut nb_failed = self.nb_failed.borrow_mut();
        *nb_failed += 1;
    }
}
