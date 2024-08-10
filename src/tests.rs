pub use helpers::*;

use aes::Aes128;
use proptest::prelude::*;

use crate::Id;

fn id_to_string_and_back<const TAG: u64>(id: i64, cipher: &Aes128) -> crate::Result<i64> {
    Id::<TAG>::new(id, cipher)
        .to_string()
        .parse()
        .and_then(|id: Id<TAG>| id.to_raw(cipher))
}

#[test]
fn id_decodes_back() {
    prop_test!(&(any::<i64>(), any_cipher()), |(id, cipher)| {
        let parsed = id_to_string_and_back::<0>(id, &cipher);
        prop_assert!(parsed.is_ok());
        prop_assert_eq!(id, parsed.unwrap());
        Ok(())
    });
}

mod helpers {
    use aes::{cipher::KeyInit, Aes128};

    use proptest::{
        prelude::*,
        test_runner::{Config, TestCaseResult, TestRunner},
    };

    pub fn any_cipher() -> impl Strategy<Value = Aes128> {
        any::<[u8; 16]>().prop_map(|key| Aes128::new(&key.into()))
    }

    pub fn __run_tests<S: Strategy>(
        config: Config,
        strategy: &S,
        test: impl Fn(S::Value) -> TestCaseResult,
    ) {
        let mut runner = TestRunner::new(config);
        match runner.run(strategy, test) {
            Ok(()) => (),
            Err(e) => panic!("{}\n{}", e, runner),
        }
    }

    macro_rules! prop_test {
        ($strategy:expr, $test:expr, $config:expr) => {
            let mut config = $config;
            config.source_file = Some(file!());
            $crate::tests::__run_tests(config, $strategy, $test);
        };
        ($strategy:expr, $test:expr) => {
            $crate::tests::prop_test!($strategy, $test, ::proptest::test_runner::Config::default())
        };
    }

    pub(crate) use prop_test;
}
