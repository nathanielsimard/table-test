#[macro_export]
macro_rules! table_test {
    ($v:expr) => {
        $crate::table::Table::new($v)
    };
}
