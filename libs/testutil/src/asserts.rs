#[macro_export]
macro_rules! assert_variant {
    ($val:expr, $pat:pat) => {
        assert!(matches!($val, $pat));
    };
}
