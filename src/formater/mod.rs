pub mod colorful_formater;
use self::colorful_formater::ColorfulFormater;

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

pub fn new_colorful() -> Box<Formater> {
    Box::new(ColorfulFormater::new())
}

#[cfg(test)]
pub mod mock;
