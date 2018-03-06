use std::rc::Rc;
use std::cell::RefCell;
use std::fmt::Debug;
use test_case::asserter::Asserter;
use formater;

pub struct TestCase {
    asserter: Asserter,
}

impl TestCase {
    pub fn new(failed: Rc<RefCell<usize>>) -> TestCase {
        let formater = formater::new_colorful();
        TestCase {
            asserter: Asserter::new(failed, formater),
        }
    }

    pub fn given(mut self, given: &str) -> TestCase {
        self.asserter
            .add_comment("Given".to_string(), given.to_string());
        self
    }

    pub fn when(mut self, when: &str) -> TestCase {
        self.asserter
            .add_comment("When".to_string(), when.to_string());
        self
    }

    pub fn then(mut self, then: &str) -> TestCase {
        self.asserter
            .add_comment("Then".to_string(), then.to_string());
        self
    }

    pub fn custom(mut self, tag: &str, comment: &str) -> TestCase {
        self.asserter
            .add_comment(tag.to_string(), comment.to_string());
        self
    }

    pub fn description(mut self, comment: &str) -> TestCase {
        self.asserter
            .add_comment("Description".to_string(), comment.to_string());
        self
    }

    pub fn assert_eq<T: PartialEq + Debug>(mut self, expected: T, actual: T) -> TestCase {
        self.asserter.assert_eq(expected, actual);
        self
    }

    pub fn assert_ne<T: PartialEq + Debug>(mut self, expected: T, actual: T) -> TestCase {
        self.asserter.assert_ne(expected, actual);
        self
    }
}
