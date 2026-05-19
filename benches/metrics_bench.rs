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

// ============================================================
// v0.9.4 additions: labels, histogram, exporters, cached-vs-global.
// ============================================================

fn labels_benchmarks(c: &mut Criterion) {
    use metrics_lib::LabelSet;
    let mut group = c.benchmark_group("labels");

    group.bench_function("from_array_2pairs", |b| {
        b.iter(|| black_box(LabelSet::from([("method", "GET"), ("status", "200")])))
    });

    group.bench_function("from_array_4pairs", |b| {
        b.iter(|| {
            black_box(LabelSet::from([
                ("method", "GET"),
                ("status", "200"),
                ("region", "us-east-1"),
                ("env", "prod"),
            ]))
        })
    });

    group.bench_function("to_prometheus_3pairs", |b| {
        let l = LabelSet::from([("a", "1"), ("b", "2"), ("c", "3")]);
        b.iter(|| black_box(l.to_prometheus()))
    });

    group.bench_function("hash_eq", |b| {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let l = LabelSet::from([("a", "1"), ("b", "2"), ("c", "3")]);
        b.iter(|| {
            let mut h = DefaultHasher::new();
            l.hash(&mut h);
            black_box(h.finish())
        })
    });

    group.finish();
}

#[cfg(feature = "histogram")]
fn histogram_benchmarks(c: &mut Criterion) {
    use metrics_lib::Histogram;
    let mut group = c.benchmark_group("histogram");

    let h = Histogram::default_seconds();
    group.bench_function("observe_default_seconds", |b| {
        b.iter(|| h.observe(black_box(0.05)))
    });

    let h = Histogram::with_buckets([0.01, 0.05, 0.1, 0.5, 1.0]);
    group.bench_function("observe_5buckets", |b| {
        b.iter(|| h.observe(black_box(0.07)))
    });

    let h = Histogram::with_buckets([
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ]);
    group.bench_function("observe_11buckets_uniform", |b| {
        let mut i: u64 = 0;
        b.iter(|| {
            i = i.wrapping_add(1);
            let v = 0.001 + ((i % 100) as f64) * 0.1;
            h.observe(black_box(v))
        })
    });

    // Concurrent observe: same histogram, multiple writers.
    group.bench_function("observe_concurrent_4_threads", |b| {
        let h = Arc::new(Histogram::default_seconds());
        b.iter(|| {
            let handles: Vec<_> = (0..4)
                .map(|tid| {
                    let h = Arc::clone(&h);
                    std::thread::spawn(move || {
                        for i in 0..1000 {
                            let v = 0.005 + ((i + tid) % 10) as f64 * 0.05;
                            h.observe(v);
                        }
                    })
                })
                .collect();
            for handle in handles {
                handle.join().unwrap();
            }
        })
    });

    // Quantile estimation (read path).
    let h = Histogram::default_seconds();
    for _ in 0..10_000 {
        h.observe(0.05);
    }
    group.bench_function("quantile_p95", |b| {
        b.iter(|| black_box(h.quantile(black_box(0.95))))
    });

    group.bench_function("snapshot", |b| b.iter(|| black_box(h.snapshot())));

    group.finish();
}

fn exporter_benchmarks(c: &mut Criterion) {
    use metrics_lib::{exporters, Counter, Gauge, LabelSet, Registry, Timer, Unit};
    let mut group = c.benchmark_group("exporters");

    // Build a representative populated registry once.
    let registry = Registry::new();
    registry.describe_counter("http_requests", "Total HTTP requests", Unit::Custom("1"));
    registry.describe_gauge("inflight", "Inflight requests", Unit::Custom("1"));
    registry.describe_timer("rpc_latency", "RPC latency", Unit::Seconds);
    for status in ["200", "400", "500"] {
        for method in ["GET", "POST", "DELETE"] {
            let labels = LabelSet::from([("method", method), ("status", status)]);
            let c: std::sync::Arc<Counter> =
                registry.get_or_create_counter_with("http_requests", &labels);
            c.add(42);
            let g: std::sync::Arc<Gauge> = registry.get_or_create_gauge_with("inflight", &labels);
            g.set(7.0);
            let t: std::sync::Arc<Timer> =
                registry.get_or_create_timer_with("rpc_latency", &labels);
            t.record_ns(1234);
        }
    }
    #[cfg(feature = "histogram")]
    {
        registry.configure_histogram(
            "rpc_duration_seconds",
            [0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0],
        );
        registry.describe_histogram("rpc_duration_seconds", "RPC duration", Unit::Seconds);
        let h = registry.get_or_create_histogram("rpc_duration_seconds");
        for i in 0..1_000 {
            h.observe(0.001 + (i % 100) as f64 * 0.005);
        }
    }

    group.bench_function("prometheus_render", |b| {
        b.iter(|| black_box(exporters::prometheus::render(&registry)))
    });

    group.bench_function("openmetrics_render", |b| {
        b.iter(|| black_box(exporters::openmetrics::render(&registry)))
    });

    #[cfg(feature = "serde")]
    group.bench_function("json_render", |b| {
        b.iter(|| black_box(exporters::json::render(&registry)))
    });

    #[cfg(feature = "statsd")]
    group.bench_function("statsd_render", |b| {
        use std::net::UdpSocket;
        let send = UdpSocket::bind("127.0.0.1:0").unwrap();
        let sink = exporters::statsd::StatsdSink::with_socket(send, "127.0.0.1:0".parse().unwrap());
        b.iter(|| black_box(sink.render(&registry)))
    });

    #[cfg(feature = "otlp")]
    group.bench_function("otlp_render", |b| {
        b.iter(|| black_box(exporters::otlp::render(&registry, "bench-service")))
    });

    group.finish();
}

fn cached_vs_global_benchmarks(c: &mut Criterion) {
    init();
    let mut group = c.benchmark_group("cached_vs_global");

    // Cached handle: the recommended hot-path pattern.
    let cached: std::sync::Arc<metrics_lib::Counter> = metrics().counter("cached_handle_bench");
    group.bench_function("counter_inc_cached_handle", |b| b.iter(|| cached.inc()));

    // Global lookup: `metrics().counter("name").inc()` per call. Includes
    // the registry `RwLock::read()` + `HashMap::get(&str)` + `Arc::clone()`
    // path. This is the realistic cost for code that hasn't cached the
    // handle.
    group.bench_function("counter_inc_global_lookup", |b| {
        b.iter(|| metrics().counter(black_box("global_lookup_bench")).inc())
    });

    // Labeled global lookup — each call also allocates the `(String,
    // LabelSet)` composite key.
    let labels = metrics_lib::LabelSet::from([("k", "v")]);
    group.bench_function("counter_inc_global_labeled_lookup", |b| {
        b.iter(|| {
            metrics()
                .counter_with(black_box("labeled_lookup_bench"), &labels)
                .inc()
        })
    });

    group.finish();
}

criterion_group!(
    benches,
    counter_benchmarks,
    gauge_benchmarks,
    timer_benchmarks,
    rate_meter_benchmarks,
    global_metrics_benchmarks,
    scaling_benchmarks,
    labels_benchmarks,
    exporter_benchmarks,
    cached_vs_global_benchmarks,
);

#[cfg(feature = "histogram")]
criterion_group!(histogram_benches, histogram_benchmarks);

#[cfg(feature = "histogram")]
criterion_main!(benches, histogram_benches);

#[cfg(not(feature = "histogram"))]
criterion_main!(benches);
