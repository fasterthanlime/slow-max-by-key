use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day16::{with_state, Network};

fn bench_run(c: &mut Criterion) {
    let net = Network::new();
    with_state(&net, |state| {
        c.bench_function("manual", |b| {
            b.iter(|| {
                black_box(state.run_manual());
            })
        });
        c.bench_function("max_by_key", |b| {
            b.iter(|| {
                black_box(state.run_max_by_key());
            })
        });
    });
}

criterion_group!(benches, bench_run);
criterion_main!(benches);
