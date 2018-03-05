use std::rc::Rc;
use std::cell::RefCell;
use validator::asserter::Asserter;
use formater;
use validator::asserter::Precision;

#[derive(Clone)]
pub struct Given {
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
    pub fn new(failed: Rc<RefCell<usize>>, value: String) -> Given {
        Given {
            failed: failed,
            value: value,
        }
    }
}

impl When {
    pub fn new(given: Given) -> When {
        When {
            given: given,
            value: String::new(),
        }
    }

    pub fn when(mut self, when: &str) -> Then {
        self.value = when.to_string();
        Then::new(self)
    }
}

impl Then {
    pub fn new(when: When) -> Then {
        Then { when: when }
    }

    pub fn then(self, then: &str) -> Asserter {
        let mut precisions = vec![];

        precisions.push(Precision{
            tag: "Given".to_string(),
            comment: self.when.given.value,
        });

        precisions.push(Precision{
            tag: "When".to_string(),
            comment: self.when.value,
        });

        precisions.push(Precision{
            tag: "Then".to_string(),
            comment: then.to_string(),
        });

        Asserter::new(
            precisions,
            self.when.given.failed,
            formater::new_colorful(),
        )
    }
}
