#[allow(unused)]
macro_rules! run_benchmarks {
    ($($fn:ident),+ $(,)?) => {
        fn main() {
            use ::squint::tag;
            $crate::macros::run_benchmarks!(
                fns = [$( $fn, )+],
                tags = [0, tag("user")],
                ids = [0, i64::MAX / 2, i64::MAX]
            );
            main()
        }
    };
    (fns = [$($fn:ident),+ $(,)?], tags = $tags:tt, ids = $ids:tt) => {
        ::criterion::criterion_main!(benches);
        ::criterion::criterion_group!(benches, run_benchmarks);

        fn run_benchmarks(c: &mut ::criterion::Criterion) {
            use ::aes::cipher::KeyInit;
            let key = [0; 16];
            let cipher = ::aes::Aes128::new(&key.into());
            $({
                let mut g = c.benchmark_group(stringify!($fn));
                $crate::macros::run_benchmarks!(@_ &mut g, $fn, $tags, $ids, &cipher);
                g.finish();
            })+
        }
    };
    (@_ $group:expr, $fn:ident, [$($tag:expr),+ $(,)?], $ids:tt, $cipher:expr) => {
        $(
            $crate::macros::run_benchmarks!(@_ $group, $fn, $tag, $ids, $cipher);
        )+
    };
    (@_ $group:expr, $fn:ident, $tag:expr, [$($id:expr),+ $(,)?], $cipher:expr) => {
        $(
            $group.bench_with_input(
                ::criterion::BenchmarkId::new(stringify!($tag), stringify!($id)),
                &$id,
                |b, &id| b.iter($fn::<{ $tag }>(id, $cipher))
            );
        )+
    };
}

#[allow(unused)]
pub(crate) use run_benchmarks;
