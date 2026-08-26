use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use limit_order_book::simulator::{
    GeneratorConfig, generate_crossing_orders, generate_two_sided_orders, run_scenario,
};
use limit_order_book::{Order, OrderBook, Side};
use std::hint::black_box;

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

fn bench_hot_path_operations(c: &mut Criterion) {
    c.bench_function("add_one_resting_order", |b| {
        b.iter(|| {
            let mut book = OrderBook::new();

            book.add_order(black_box(Order::new(1, Side::Buy, 100, 10)))
                .expect("order should be accepted");
        });
    });

    c.bench_function("single_trade", |b| {
        b.iter_batched(
            || {
                let mut book = OrderBook::new();

                // Setup resting liquidity outside the measured closure.
                book.add_order(Order::new(1, Side::Buy, 100, 10))
                    .expect("setup order should be accepted");

                book
            },
            |mut book| {
                // Measures only the crossing sell order.
                book.add_order(black_box(Order::new(2, Side::Sell, 100, 5)))
                    .expect("crossing order should be accepted");
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("multi_level_sweep", |b| {
        b.iter_batched(
            || {
                let mut book = OrderBook::new();

                // Setup multiple ask levels outside the measured closure.
                book.add_order(Order::new(1, Side::Sell, 100, 5)).unwrap();
                book.add_order(Order::new(2, Side::Sell, 101, 5)).unwrap();
                book.add_order(Order::new(3, Side::Sell, 102, 5)).unwrap();

                book
            },
            |mut book| {
                // Measures one buy order sweeping three ask levels.
                book.add_order(black_box(Order::new(4, Side::Buy, 102, 12)))
                    .expect("sweep order should be accepted");
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("cancel_order", |b| {
        b.iter_batched(
            || {
                let mut book = OrderBook::new();

                // Setup one resting order outside the measured closure.
                book.add_order(Order::new(1, Side::Buy, 100, 10))
                    .expect("setup order should be accepted");

                book
            },
            |mut book| {
                // Measures removing an active resting order.
                book.cancel_order(black_box(1))
                    .expect("cancel should succeed");
            },
            BatchSize::SmallInput,
        );
    });

    c.bench_function("modify_order", |b| {
        b.iter_batched(
            || {
                let mut book = OrderBook::new();

                // Setup one resting order outside the measured closure.
                book.add_order(Order::new(1, Side::Buy, 100, 10))
                    .expect("setup order should be accepted");

                book
            },
            |mut book| {
                // Measures changing price and quantity of an active order.
                book.modify_order(black_box(1), black_box(101), black_box(7))
                    .expect("modify should succeed");
            },
            BatchSize::SmallInput,
        );
    });
}

criterion_group!(
    benches,
    bench_two_sided_synthetic,
    bench_crossing_synthetic,
    bench_hot_path_operations
);
criterion_main!(benches);
