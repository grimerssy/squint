#[allow(unused)]
macro_rules! run_benchmarks {
    ($($fn:ident),+ $(,)?) => {
        fn main() {
            use ::squint::tag;
            $crate::macros::run_benchmarks!(
                fns = [$( $fn, )+],
                params = [0, { tag("user") }]
            );
            main()
        }
    };
    (fns = $fns:tt, params = $params:tt) => {
        $crate::macros::run_benchmarks!(
            fns = $fns,
            params = $params,
            args = [0, i64::MAX / 2, i64::MAX]
        );
    };
    (fns = [$($fn:ident),+ $(,)?], params = $params:tt, args = $args:tt) => {
        ::criterion::criterion_main!(benches);
        ::criterion::criterion_group!(benches, run_benchmarks);

        fn run_benchmarks(c: &mut ::criterion::Criterion) {
            use ::aes::cipher::KeyInit;
            let key = [0; 16];
            let cipher = ::aes::Aes128::new(&key.into());
            $({
                let file = file!()
                    .strip_prefix("benches/")
                    .and_then(|f| f.strip_suffix(".rs"))
                    .unwrap();
                let function = stringify!($fn);
                let group = format!("{file}/{function}");
                let mut group = c.benchmark_group(&group);
                $crate::macros::run_benchmarks!(@_ &mut group, $fn, $params, $args, &cipher);
                group.finish();
            })+
        }
    };
    (@_ $group:expr, $fn:ident, [$($param:tt),+ $(,)?], $args:tt, $cipher:expr) => {
        $(
            $crate::macros::run_benchmarks!(@_ $group, $fn, $param, $args, $cipher);
        )+
    };
    (@_ $group:expr, $fn:ident, $param:tt, [$($arg:expr),+ $(,)?], $cipher:expr) => {
        $(
            $group.bench_with_input(
                ::criterion::BenchmarkId::new(stringify!($param), stringify!($arg)),
                &$arg,
                |b, &arg| b.iter($fn::<$param>(arg, $cipher))
            );
        )+
    };
}

#[allow(unused)]
pub(crate) use run_benchmarks;
