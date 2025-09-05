# Performance Tuning

Practical tips to extract the maximum throughput and lowest latency when using `metrics-lib` in production.

## CPU Affinity and Scheduling

- Pin hot workers to physical cores to reduce context switches.
- Linux (example, taskset):
```bash
# pin PID 12345 to cores 2-3
sudo taskset -cp 2,3 12345
```
- Tokio: prefer bounded worker threads and avoid oversubscription.

## NUMA Awareness

- Keep producer and consumer threads on the same NUMA node as much as possible.
- Allocate long-lived metric structures early (during init) to the node that will touch them most.
- Tools: `numactl`, `hwloc` to inspect and enforce placement.

## Denormals and Frequency Scaling

- Disable CPU frequency scaling (use performance governor) for stable benches.
- Avoid denormals in gauge math where possible.
- Linux:
```bash
# set performance governor (modern systems)
echo performance | sudo tee /sys/devices/system/cpu/cpu*/cpufreq/scaling_governor
```

## Lock Contention

- Prefer batched updates (`add`, `record_batch`, `tick_n`) to amortize costs under bursts.
- Use `RateMeter` for admission rather than central mutexes.

## Memory and Cache

- Pre-register metric handles; avoid creating/dropping in hot paths.
- Avoid high-cardinality metric names; keep names `'static` where possible.
- All core types are `#[repr(align(64))]` to avoid false sharing; still, avoid placing unrelated hot data in the same cache line.

## Async Considerations

- Use `AsyncMetricBatch` to coalesce updates.
- Avoid fine-grained awaits around individual metric ops.

## Criterion Settings for Stable Results

- Increase warmup/measurement/sample sizes on metal runners:
```bash
cargo bench -- -w 3.0 -m 5.0 -n 100
```
- Run multiple times and compare medians.

## Exporting Without Perturbing Hot Paths

- Use a background task to snapshot metrics and ship off-process.
- Bound queues; drop or sample on overload.

## Checklists

- CPU governor set to `performance` (or equivalent).
- Fixed rustc, toolchain, and target features.
- Benches run on quiescent system (no heavy background jobs).
