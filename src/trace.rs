use std::fs;
use serde_json::Value;

pub fn parse_trace_json(trace_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(trace_path)?;
    let json: Value = serde_json::from_str(&data)?;
    if let Some(events) = json.get("traceEvents").and_then(|v| v.as_array()) {
        let mut times = vec![];
        for e in events {
            if e.get("name") == Some(&Value::String("RunTask".to_string())) {
                if let Some(dur) = e.get("dur").and_then(|d| d.as_u64()) {
                    times.push(dur as f64 / 1000.0);
                }
            }
        }
        times.sort_by(|a, b| b.partial_cmp(a).unwrap());
        println!("Top 5 RunTask durations (ms):");
        for dur in times.iter().take(5) {
            println!("- {:.2} ms", dur);
        }
    }
    Ok(())
}
