use binid::{FirstResolution, Path};
use criterion::{criterion_group, criterion_main, Criterion};

fn concat_benchmark(c: &mut Criterion) {
    let mut path = Path::new(FirstResolution::new(1).unwrap());

    path.add_resolution(2);
    path.add_resolution(3);
    path.add_resolution(4);
    path.add_resolution(5);
    path.add_resolution(1);
    path.add_resolution(2);

    c.bench_function("id_concat", |bencher| {
        bencher.iter(|| {
            path.id_concat();
        });
    });
}

criterion_group!(benches, concat_benchmark);
criterion_main!(benches);