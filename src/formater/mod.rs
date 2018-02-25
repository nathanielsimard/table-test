pub mod formater;

pub trait Formater {
    fn format_passed_test(&self, inputs: &String, test_name: &String, expected: &String) -> String;
    fn format_failed_test(
        &self,
        given: &String,
        when: &String,
        then: &String,
        expected: &String,
        actual: &String,
    ) -> String;
}

pub fn new() -> Box<Formater> {
    Box::new(formater::ColorfulFormater::new())
}
