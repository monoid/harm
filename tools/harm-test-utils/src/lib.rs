mod db;
pub use db::*;

#[macro_export]
macro_rules! test_cases {
    ($db_data:expr; $($test_name:ident, $expr:expr, $($key:expr),+;)*) => {
        $(
            #[test]
            fn $test_name() {
                extern crate alloc;
                let inst: alloc::vec::Vec<_> = $expr.encode().collect();
                assert_eq!(
                    inst,
                    [$(($crate::db($db_data, $key), None)),*]
        );
            }
        )*
    };
    ($db_data:expr, $completeness_test:ident; $($test_name:ident, $expr:expr, $($key:expr),+;)*) => {
        #[test]
        fn $completeness_test() {
            extern crate alloc;
            let mut db = $crate::Db::new($db_data).unwrap_or_else(|e| panic!("{e}"));
            let keys = [$($($key,)+)*].into_iter().collect::<alloc::collections::BTreeSet<_>>();
            let db_keys = db.keys().collect::<alloc::collections::BTreeSet<_>>();
            let mut untested_keys = db_keys.difference(&keys).collect::<alloc::vec::Vec<_>>();
            untested_keys.sort_unstable();
            assert!(
                untested_keys.is_empty(),
                "The following keys were not tested: {:#?}",
                untested_keys
            );
        }

        test_cases!($db_data; $($test_name, $expr, $($key),+;)*);
    }
}

#[macro_export]
macro_rules! inst {
    ($([$b0:expr, $b1:expr, $b2:expr, $b3:expr]),*) => {
        [$((InstructionCode([$b0, $b1, $b2, $b3]), None)),*]
    };
    ($($code:literal),*) => {
        [$((InstructionCode::from_u32($code), None)),*]
    }
}
