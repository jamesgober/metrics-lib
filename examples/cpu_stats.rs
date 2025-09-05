use metrics_lib::metrics;

fn main() {
    // Initialize metrics core if needed by your app
    metrics_lib::init();

    let sys = metrics().system();

    // System CPU usage
    let used = sys.cpu_used();
    let free = sys.cpu_free();
    let load = sys.load_avg();
    let threads = sys.thread_count();
    let fds = sys.fd_count();

    // Process CPU usage
    let proc_cpu = sys.process_cpu_used();

    println!("=== CPU Overview ===");
    println!("system.cpu.used      : {:>6.2}%", used);
    println!("system.cpu.free      : {:>6.2}%", free);
    println!("system.load.1m       : {:>6.2}", load);
    println!("process.cpu.used     : {:>6.2}%", proc_cpu);
    println!("process.threads      : {}", threads);
    println!("process.fds          : {}", fds);
}
