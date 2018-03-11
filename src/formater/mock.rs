use formater::Formater;
use mock_it::Mock;
use mock_it::Matcher;
use mock_it::Matcher::Val;

#[derive(Clone)]
pub struct FormaterMock {
    pub format_one_line: Mock<(Matcher<String>, Matcher<String>), String>,
    pub format_passed_test_header: Mock<(), String>,
    pub format_failed_test_header: Mock<(), String>,
    pub format_diff: Mock<(String, String), String>,
}

impl FormaterMock {
    pub fn new() -> FormaterMock {
        FormaterMock {
            format_one_line: Mock::new(String::from("")),
            format_passed_test_header: Mock::new(String::from("")),
            format_failed_test_header: Mock::new(String::from("")),
            format_diff: Mock::new(String::from("")),
        }
    }
}

impl Formater for FormaterMock {
    fn format_one_line(&self, tag: &String, comment: &String) -> String {
        self.format_one_line.called((Val(tag.clone()), Val(comment.clone())))
    }
    fn format_passed_test_header(&self) -> String {
        self.format_passed_test_header.called(())
    }
    fn format_failed_test_header(&self) -> String {
        self.format_failed_test_header.called(())
    }
    fn format_diff(&self, expected: &String, actual: &String) -> String {
        self.format_diff.called((expected.clone(), actual.clone()))
    }
}
