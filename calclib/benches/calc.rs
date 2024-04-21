use std::time::Duration;

use calclib::Calc;
use criterion::{criterion_group, criterion_main, Criterion};

fn benchmark_add(c: &mut Criterion) {
    println!("Running benchmark for add...");
    let mut group = c.benchmark_group("add");
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);
    group.bench_function("Calc::add", |b| {
        b.iter(|| {
            let mut calc = Calc::new(0.0);
            for i in 1..10000 {
                calc.add(i as f64);
            }
        })
    });
    group.finish();
}

fn benchmark_sub(c: &mut Criterion) {
    println!("Running benchmark for sub...");
    let mut group = c.benchmark_group("sub");
    group.measurement_time(Duration::from_secs(2));
    group.sample_size(20);
    group.bench_function("Calc::sub", |b| {
        b.iter(|| {
            let mut calc = Calc::new(0.0);
            for i in 1..10000 {
                calc.sub(i as f64);
            }
        })
    });
    group.finish();
}

fn benchmarks(c: &mut Criterion) {
    benchmark_add(c);
    benchmark_sub(c);
}

criterion_group!(benches, benchmarks);
criterion_main!(benches);
