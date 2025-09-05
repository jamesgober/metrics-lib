use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use metrics_lib::{init, metrics, Counter, Gauge, RateMeter, Timer};
use std::sync::Arc;
use std::time::Duration;

fn counter_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("counter");

    // Single-threaded benchmarks
    let counter = Counter::new();
    group.bench_function("increment", |b| b.iter(|| counter.inc()));

    group.bench_function("add", |b| b.iter(|| counter.add(black_box(5))));

    group.bench_function("get", |b| b.iter(|| counter.get()));

    // Multi-threaded benchmark
    group.bench_function("concurrent_increment", |b| {
        let counter = Arc::new(Counter::new());
        b.iter(|| {
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let counter = Arc::clone(&counter);
                    std::thread::spawn(move || {
                        for _ in 0..100 {
                            counter.inc();
                        }
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    });

    // High-contention: multiple threads add in bursts
    group.bench_function("concurrent_add_bursts_4_threads", |b| {
        b.iter(|| {
            let c = Arc::new(Counter::new());
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let c = Arc::clone(&c);
                    std::thread::spawn(move || {
                        for _ in 0..1000 {
                            c.add(5);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });

    group.finish();
}

fn gauge_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("gauge");

    let gauge = Gauge::new();

    group.bench_function("set", |b| b.iter(|| gauge.set(black_box(42.5))));

    group.bench_function("add", |b| b.iter(|| gauge.add(black_box(1.5))));

    group.bench_function("get", |b| b.iter(|| gauge.get()));

    group.bench_function("set_min", |b| b.iter(|| gauge.set_min(black_box(10.0))));

    group.bench_function("set_max", |b| b.iter(|| gauge.set_max(black_box(100.0))));

    // High-contention: concurrent add and set operations
    group.bench_function("concurrent_add_set_4_threads", |b| {
        b.iter(|| {
            let g = Arc::new(Gauge::new());
            let handles: Vec<_> = (0..4)
                .map(|tid| {
                    let g = Arc::clone(&g);
                    std::thread::spawn(move || {
                        for i in 0..2000 {
                            if (i + tid) % 4 == 0 {
                                g.set((i as f64) * 0.001);
                            } else {
                                g.add(0.1);
                            }
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });

    group.finish();
}

fn timer_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("timer");

    let timer = Timer::new();
    let duration = Duration::from_nanos(100);

    group.bench_function("record", |b| b.iter(|| timer.record(black_box(duration))));

    group.bench_function("record_ns", |b| b.iter(|| timer.record_ns(black_box(100))));

    group.bench_function("start_stop", |b| {
        b.iter(|| {
            let guard = timer.start();
            guard.stop();
        })
    });

    group.bench_function("raii_timing", |b| {
        b.iter(|| {
            let _guard = timer.start();
            black_box(());
        })
    });

    group.bench_function("stats", |b| {
        // Pre-populate with some data
        for i in 0..1000 {
            timer.record_ns(i * 1000);
        }
        b.iter(|| timer.stats())
    });

    group.finish();
}

fn rate_meter_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("rate_meter");

    let rate_meter = RateMeter::new();

    group.bench_function("tick", |b| b.iter(|| rate_meter.tick()));

    group.bench_function("tick_n", |b| b.iter(|| rate_meter.tick_n(black_box(5))));

    group.bench_function("rate", |b| {
        // Pre-populate with some data
        for _ in 0..100 {
            rate_meter.tick();
        }
        b.iter(|| rate_meter.rate())
    });

    // High-contention benchmark: concurrent tick_n across multiple threads
    group.bench_function("tick_n_concurrent_4_threads", |b| {
        b.iter(|| {
            let meter = Arc::new(RateMeter::new());
            let handles: Vec<_> = (0..4)
                .map(|_| {
                    let meter = Arc::clone(&meter);
                    std::thread::spawn(move || {
                        for _ in 0..1000 {
                            meter.tick_n(5);
                        }
                    })
                })
                .collect();

            for h in handles {
                h.join().unwrap();
            }
        })
    });

    group.finish();
}

fn global_metrics_benchmarks(c: &mut Criterion) {
    init();
    let mut group = c.benchmark_group("global_metrics");

    group.bench_function("counter_access", |b| {
        b.iter(|| {
            metrics().counter(black_box("test_counter")).inc();
        })
    });

    group.bench_function("gauge_access", |b| {
        b.iter(|| {
            metrics()
                .gauge(black_box("test_gauge"))
                .set(black_box(42.5));
        })
    });

    group.bench_function("timer_access", |b| {
        b.iter(|| {
            let timer = metrics().timer(black_box("test_timer"));
            let _timer = timer.start();
            black_box(());
        })
    });
    group.bench_function("mixed_operations", |b| {
        b.iter(|| {
            metrics().counter("requests").inc();
            metrics().gauge("cpu").set(85.2);
            let timer = metrics().timer("api");
            let _timer = timer.start();
            metrics().rate("calls").tick();
        })
    });

    group.finish();
}

fn scaling_benchmarks(c: &mut Criterion) {
    let mut group = c.benchmark_group("scaling");

    for threads in [1, 2, 4, 8, 16].iter() {
        let counter = Arc::new(Counter::new());
        group.bench_with_input(
            BenchmarkId::new("counter_threads", threads),
            threads,
            |b, &thread_count| {
                b.iter(|| {
                    let handles: Vec<_> = (0..thread_count)
                        .map(|_| {
                            let counter = Arc::clone(&counter);
                            std::thread::spawn(move || {
                                for _ in 0..1000 {
                                    counter.inc();
                                }
                            })
                        })
                        .collect();

                    for handle in handles {
                        handle.join().unwrap();
                    }
                });
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    counter_benchmarks,
    gauge_benchmarks,
    timer_benchmarks,
    rate_meter_benchmarks,
    global_metrics_benchmarks,
    scaling_benchmarks
);
criterion_main!(benches);
