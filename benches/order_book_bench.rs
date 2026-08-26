use criterion::{ criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use limit_order_book::simulator::{
    generate_crossing_orders, generate_two_sided_orders, run_scenario, GeneratorConfig,
};

fn bench_two_sided_synthetic(c: &mut Criterion) {
    // Pre-generate commands so we measure order book execution,
    // not synthetic data creation.
    let commands = generate_two_sided_orders(GeneratorConfig {
        order_count: 1_000,
        start_order_id: 1,
        base_price: 100,
        tick_size: 1,
        price_levels: 10,
        quantity: 5,
    });

    c.bench_function("two_sided_1000_orders", |b| {
        b.iter(|| {
            run_scenario(black_box(&commands)).expect("benchmark scenario should run");
        });
    });
}

fn bench_crossing_synthetic(c: &mut Criterion) {
    // This workload creates trades, so it measures matching pressure.
    let commands = generate_crossing_orders(GeneratorConfig {
        order_count: 1_000,
        start_order_id: 1,
        base_price: 100,
        tick_size: 1,
        price_levels: 10,
        quantity: 5,
    });

    c.bench_function("crossing_1000_orders", |b| {
        b.iter(|| {
            run_scenario(black_box(&commands)).expect("benchmark scenario should run");
        });
    });
}

criterion_group!(benches, bench_two_sided_synthetic, bench_crossing_synthetic);
criterion_main!(benches);