#[macro_export]
macro_rules! table_test {
    ($n:expr, $v:expr) => {
        $crate::table::new($n, $v.to_vec())
    };
}
