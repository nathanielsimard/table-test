pub mod colorful_formater;
use self::colorful_formater::ColorfulFormater;

pub trait Formater {
    fn format_passed_test_header(&self) -> String;
    fn format_failed_test_header(&self) -> String;
    fn format_diff(&self, expected: &String, actual: &String) -> String;
    fn format_one_line(&self, tag: &String, comment: &String) -> String;
}

pub fn new_colorful() -> Box<Formater> {
    Box::new(ColorfulFormater::new())
}

#[cfg(test)]
pub mod mock;
