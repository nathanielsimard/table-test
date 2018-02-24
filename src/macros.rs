#[macro_export]
macro_rules! table_test {
    ($n:expr, $v:expr) => {
        $crate::table::Table::new(stringify!($n), $v)
    };
}
