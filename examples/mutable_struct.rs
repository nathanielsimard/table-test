#[macro_use]
extern crate table_test;

#[derive(Debug, PartialEq)]
struct Person {
    name: String,
    surname: String,
}

impl Person {
    fn new(name: &str, surname: &str) -> Person {
        Person {
            name: name.to_string(),
            surname: surname.to_string(),
        }
    }

    fn change_name(&mut self, name: &str) {
        self.surname = name.to_string(); // Oups should be self.name
    }
}

fn main() {
    let table = table_test!(
        change_name,
        vec![
            (
                (Person::new("John", "Smith"), "Johny"),
                Person::new("Johny", "Smith"),
            ),
            (
                (Person::new("Ian", "Taylor"), "Yan"),
                Person::new("Yan", "Taylor"),
            ),
            (
                (Person::new("Justin", "Williams"), "Justine"),
                Person::new("Justine", "Williams"),
            ),
        ]
    );

    for (validator, (mut person, new_name), expected) in table {
        person.change_name(new_name);

        validator.assert_eq(expected, person);
    }
}
