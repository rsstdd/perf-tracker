use std::error::Error;
use std::fs::{self, read_to_string, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use chrono::Local;
use serde_json::{json, Value};

use crate::metrics::LighthouseMetrics;

/// Safely updates or creates `summary.json` with a new performance entry.
pub fn update_summary(
    scenario: &str,
    url: &str,
    fetch_time: &str,
    metrics: &LighthouseMetrics,
) -> io::Result<()> {
    let path = "summary.json";

    let mut entries = if Path::new(path).exists() {
        let content = read_to_string(path)?;
        serde_json::from_str::<Vec<Value>>(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    let new_entry = json!({
        "scenario": scenario,
        "url": url,
        "fetch_time": fetch_time,
        "metrics": metrics
    });

    entries.push(new_entry);

    let pretty = serde_json::to_string_pretty(&entries)?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)?;
    file.write_all(pretty.as_bytes())?;

    Ok(())
}

/// Lists all local Lighthouse JSON reports.
pub async fn list_local_reports() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let path = entry?.path();
        if path.is_file() && path.to_string_lossy().contains("lighthouse_report") {
            println!("Found report: {}", path.display());
        }
    }
    Ok(())
}

/// Prints a tabular summary of today's Lighthouse JSON reports.
pub fn summarize_local_json_reports() -> Result<(), Box<dyn Error>> {
    println!("\n=== Performance Summary Table ===");

    let today = Local::now().format("%Y-%m-%d").to_string();
    let pattern = "lighthouse_report_";

    for entry in fs::read_dir(".")? {
        let path = entry?.path();
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.starts_with(pattern) && name.ends_with(&format!("{}.json", today)) {
                let raw = fs::read_to_string(&path)?;
                let json: Value = serde_json::from_str(&raw)?;

                let scenario = name
                    .strip_prefix(pattern)
                    .unwrap_or("unknown")
                    .strip_suffix(&format!("_{}.json", today))
                    .unwrap_or("unknown");

                let perf = json["categories"]["performance"]["score"]
                    .as_f64()
                    .unwrap_or(0.0) * 100.0;
                let fcp = json["audits"]["first-contentful-paint"]["numericValue"]
                    .as_f64()
                    .unwrap_or(0.0) / 1000.0;
                let lcp = json["audits"]["largest-contentful-paint"]["numericValue"]
                    .as_f64()
                    .unwrap_or(0.0) / 1000.0;
                let tti = json["audits"]["interactive"]["numericValue"]
                    .as_f64()
                    .unwrap_or(0.0) / 1000.0;
                let tbt = json["audits"]["total-blocking-time"]["numericValue"]
                    .as_f64()
                    .unwrap_or(0.0) / 1000.0;

                println!(
                    "{:<18} | Perf: {:>5.1} | FCP: {:>4.2}s | LCP: {:>4.2}s | TTI: {:>4.2}s | TBT: {:>4.2}s",
                    scenario, perf, fcp, lcp, tti, tbt
                );
            }
        }
    }

    Ok(())
}

/// Appends an entry to `summary.json` safely (alias for update_summary).
pub fn append_to_summary_json(
    scenario: &str,
    url: &str,
    fetch_time: &str,
    metrics: &LighthouseMetrics,
) -> io::Result<()> {
    update_summary(scenario, url, fetch_time, metrics)
}
