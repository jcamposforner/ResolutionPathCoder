use binid::{FirstResolution, Path};
use criterion::{criterion_group, criterion_main, Criterion};

fn bitwise_benchmark(c: &mut Criterion) {
    let mut path = Path::new(FirstResolution::new(1).unwrap());

    path.add_resolution(2);
    path.add_resolution(3);
    path.add_resolution(4);
    path.add_resolution(5);
    path.add_resolution(1);
    path.add_resolution(2);

    c.bench_function("bitwise", |bencher| {
        bencher.iter(|| {
            path.id();
        });
    });
}

criterion_group!(benches, bitwise_benchmark);
criterion_main!(benches);