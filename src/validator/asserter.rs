use std::fmt::Debug;
use std::rc::Rc;
use std::cell::RefCell;
use formater::Formater;

pub struct Asserter {
    given: String,
    when: String,
    then: String,
    nb_failed: Rc<RefCell<usize>>,
    formater: Box<Formater>,
}

impl Asserter {
    pub fn new(
        given: String,
        when: String,
        then: String,
        failed: Rc<RefCell<usize>>,
        formater: Box<Formater>,
    ) -> Asserter {
        Asserter {
            given: given,
            when: when,
            then: then,
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
            self.formater.format_failed_test(
                &self.given,
                &self.when,
                &self.then,
                &String::from(format!("{:?}", expected)),
                &String::from(format!("{:?}", actual)),
            )
        } else {
            self.formater
                .format_passed_test(&self.given, &self.when, &self.then)
        }
    }

    fn increment_failed_counter(&self) {
        let mut nb_failed = self.nb_failed.borrow_mut();
        *nb_failed += 1;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_assert_eq() {}
}
