mod metrics;
mod report;
mod summary;
mod trace;
mod lighthouse;

use crate::metrics::LighthouseMetrics;
use crate::report::save_metrics_to_txt;
use crate::summary::{append_to_summary_json, summarize_local_json_reports};
use crate::trace::parse_trace_json;
use crate::lighthouse::fetch_lighthouse_metrics;

use chrono::Utc;
use dotenv::dotenv;

/// Runs multiple Lighthouse audits under various scenarios,
/// aggregates results, saves reports, and parses traces.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Performance Tracker starting...");

    dotenv().ok();

    const BASE_URL: &str = "https://alaskaair.com";

    let scenarios = [
        ("baseline", BASE_URL, vec![]),
        ("no-tealium", BASE_URL, vec!["*.tealiumiq.com"]),
        ("no-appd", BASE_URL, vec!["*.appdynamics.com"]),
        ("no-optimizely", BASE_URL, vec!["*.optimizely.com"]),
        ("no-header-footer", BASE_URL, vec!["*/header*", "*/footer*"]),
        ("no-quantum", BASE_URL, vec!["*.quantummetric.com"]),
    ];

    let num_runs = 3;

    for (label, url, blocked) in scenarios {
        println!("\n=== Running Scenario: {} ===", label);

        let mut total_metrics = LighthouseMetrics::default();
        let mut successful_runs = 0;

        for i in 0..num_runs {
            println!("-> Run {}/{} for {}", i + 1, num_runs, label);
            match fetch_lighthouse_metrics(label, url, &blocked).await {
                Ok(metrics) => {
                    total_metrics.add(&metrics);
                    successful_runs += 1;
                }
                Err(e) => {
                    eprintln!("❌ Run {} failed: {}", i + 1, e);
                }
            }
        }

        if successful_runs > 0 {
            total_metrics.average(successful_runs as f64);
            let metrics_in_seconds = total_metrics.to_seconds();
            let fetch_time = Utc::now().to_rfc3339();

            save_metrics_to_txt(&metrics_in_seconds, url, &fetch_time).await?;
            append_to_summary_json(label, url, &fetch_time, &metrics_in_seconds)?;

            println!("\nSummary for scenario '{}':", label);
            println!("{}", metrics_in_seconds.evaluate());

            println!("Top 5 Performance Bottlenecks:");
            for (metric, value) in metrics_in_seconds.top_offenders() {
                println!("- {}: {:.2}", metric, value);
            }

            println!("\n✅ Completed scenario: {}\n", label);
        } else {
            eprintln!("\n❌ All runs failed for scenario: {}\n", label);
        }
    }

    println!("✅ All Lighthouse scenarios completed.");

    summarize_local_json_reports()?;

    // ⚠️ Defensive: Check if "trace.json" exists before parsing
    if std::path::Path::new("trace.json").exists() {
        parse_trace_json("trace.json")?;
    } else {
        println!("⚠️ No trace.json found to parse.");
    }

    Ok(())
}
