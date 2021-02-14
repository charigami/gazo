use criterion::{criterion_group, criterion_main, Criterion};
use gazo::{ImgRGBA, PointOperations};

pub fn criterion_benchmark(c: &mut Criterion) {
    let img = ImgRGBA::from_file("images/jellyfish.png").unwrap();

    let mut group = c.benchmark_group("PointOperations");

    group.sample_size(1000);

    group.bench_function("Grayscale", |b| {
        b.iter(|| {
            let mut img_copy = img.clone();
            img_copy.grayscale();
        })
    });

    group.bench_function("Invert", |b| {
        b.iter(|| {
            let mut img_copy = img.clone();
            img_copy.invert();
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
