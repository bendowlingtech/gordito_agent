
use sysinfo::{
    System,
};

use serde_json::Value;

pub(crate) fn collect_system_metrics() -> Value {
    let mut sys = System::new_all();
    sys.refresh_all();

    let cpu_usage = sys.global_cpu_usage();
    let used_memory_bytes = sys.used_memory();

    // Convert bytes to gigabyte
    let used_memory_gb = bytes_to_gb(used_memory_bytes);

    // Return the metrics as a JSON object
    serde_json::json!({
        "cpu_usage": cpu_usage,
        "used_memory_gb": used_memory_gb,
    })
}

fn bytes_to_gb(bytes: u64) -> f64 {
    const BYTES_PER_GB: f64 = 1073741824.0;
    (bytes as f64) / BYTES_PER_GB
}