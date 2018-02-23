extern crate ansi_term;

use std::iter::Iterator;
use std::vec::IntoIter;
use std::rc::Rc;
use std::sync::Mutex;

mod validator;
use validator::Validator;
use std::fmt::Debug;

struct Table<I, E> {
    values: IntoIter<(I, E)>,
    name: String,
    failed: Rc<Mutex<usize>>,
}

fn new<I, E>(name: &str, vec: Vec<(I, E)>) -> Table<I, E> {
    Table {
        values: vec.into_iter(),
        failed: Rc::new(Mutex::new(0)),
        name: String::from(name),
    }
}

impl<I, E> Drop for Table<I, E> {
    fn drop(&mut self) {
        let failed = *self.failed.lock().unwrap();
        if failed > 0 {
            panic!("{} test failed", failed);
        }
    }
}

impl<I: Debug, E: Debug> Iterator for Table<I, E> {
    type Item = (Validator, I, E);
    fn next(&mut self) -> Option<Self::Item> {
        let items = self.values.next();

        match items {
            Some(value) => {
                let inputs = format!("{:?}", value.0);
                let result = (
                    validator::new(self.name.clone(), inputs, Rc::clone(&self.failed)),
                    value.0,
                    value.1,
                );

                Some(result)
            }
            None => None,
        }
    }
}

fn name_is_beatiful(_name: &str) -> bool {
    true
}

pub fn name_is_beatiful_test() {
    let table = new(
        "name is beatiful",
        vec![
            ("an ugly name", true),
            ("a beautiful name", true),
            ("an amazing name", false),
            ("worst name ever", false),
        ],
    );

    for (mut validator, name, expected) in table {
        let actual = name_is_beatiful(name);

        validator.assert_eq(&expected, &actual);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        name_is_beatiful_test();
    }
}
