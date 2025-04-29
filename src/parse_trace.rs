use serde_json::Value;
use std::fs;

pub fn parse_trace_json(trace_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let data = fs::read_to_string(trace_path)?;
    let json: Value = serde_json::from_str(&data)?;

    if let Some(events) = json.get("traceEvents").and_then(|e| e.as_array()) {
        let mut main_thread_times = Vec::new();

        for event in events {
            if let Some(name) = event.get("name").and_then(|n| n.as_str()) {
                if name == "RunTask" {
                    if let Some(dur) = event.get("dur").and_then(|d| d.as_u64()) {
                        main_thread_times.push(dur as f64 / 1000.0); // convert microseconds to milliseconds
                    }
                }
            }
        }

        if !main_thread_times.is_empty() {
            main_thread_times.sort_by(|a, b| b.partial_cmp(a).unwrap());

            println!("Top 5 Main Thread Task Durations (ms):");
            for task_duration in main_thread_times.iter().take(5) {
                println!("- {:.2} ms", task_duration);
            }
        } else {
            println!("No RunTask events found in trace.");
        }
    } else {
        println!("No traceEvents found.");
    }

    Ok(())
}
