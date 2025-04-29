use std::error::Error;
use std::process::Command;
use chrono::Local;
use serde_json::Value;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde_json::to_string_pretty;
use crate::metrics::LighthouseMetrics;

/// Runs Lighthouse and extracts performance metrics.
///
/// # Arguments
///
/// * `label` - Name of the scenario (for file naming).
/// * `url` - URL to run Lighthouse against.
/// * `blocked_patterns` - Optional URL patterns to block.
///
/// # Returns
///
/// * `Ok(LighthouseMetrics)` on success.
/// * `Err(Box<dyn Error>)` on failure.
pub async fn fetch_lighthouse_metrics(label: &str, url: &str, blocked_patterns: &[&str]) -> Result<LighthouseMetrics, Box<dyn Error>> {
    let mut args = vec![
        url,
        "--output=json",
        "--output-path=stdout",
        "--quiet",
        "--window-size=1000,1000",
        "--preset=desktop",
        "--headless",
        "--only-categories=performance,accessibility,seo,best-practices",
        "--save-assets",
    ];

    for pattern in blocked_patterns {
        args.push("--blocked-url-patterns");
        args.push(pattern);
    }

    let output = Command::new("lighthouse")
        .args(&args)
        .output()?;

    if !output.status.success() {
        return Err(format!("Lighthouse command failed with status: {}", output.status).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let json: Value = serde_json::from_str(&stdout)?;

    let formatted_json = to_string_pretty(&json)?;
    let date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("lighthouse_report_{}_{}.json", label, date);

    let mut file = File::create(&file_name).await?;
    file.write_all(formatted_json.as_bytes()).await?;

    println!("✅ Saved report: {}", file_name);

    Ok(extract_metrics(&json))
}

/// Parses performance metrics from Lighthouse JSON.
fn extract_metrics(json: &Value) -> LighthouseMetrics {
    LighthouseMetrics {
        first_contentful_paint: json["audits"]["first-contentful-paint"]["numericValue"].as_f64().unwrap_or(0.0),
        largest_contentful_paint: json["audits"]["largest-contentful-paint"]["numericValue"].as_f64().unwrap_or(0.0),
        time_to_interactive: json["audits"]["interactive"]["numericValue"].as_f64().unwrap_or(0.0),
        total_blocking_time: json["audits"]["total-blocking-time"]["numericValue"].as_f64().unwrap_or(0.0),
        cumulative_layout_shift: json["audits"]["cumulative-layout-shift"]["numericValue"].as_f64().unwrap_or(0.0),
        speed_index: json["audits"]["speed-index"]["numericValue"].as_f64().unwrap_or(0.0),
        performance_score: json["categories"]["performance"]["score"].as_f64().unwrap_or(0.0) * 100.0,
        first_meaningful_paint: json["audits"]["first-meaningful-paint"]["numericValue"].as_f64().unwrap_or(0.0),
        first_cpu_idle: json["audits"]["first-cpu-idle"]["numericValue"].as_f64().unwrap_or(0.0),
        max_potential_fid: json["audits"]["max-potential-fid"]["numericValue"].as_f64().unwrap_or(0.0),
        estimated_input_latency: json["audits"]["estimated-input-latency"]["numericValue"].as_f64().unwrap_or(0.0),
        server_response_time: json["audits"]["server-response-time"]["numericValue"].as_f64().unwrap_or(0.0),
        javascript_bootup_time: json["audits"]["bootup-time"]["numericValue"].as_f64().unwrap_or(0.0),
        total_byte_weight: json["audits"]["total-byte-weight"]["numericValue"].as_f64().unwrap_or(0.0),
        render_blocking_resources: json["audits"]["render-blocking-resources"]["numericValue"].as_f64().unwrap_or(0.0),
        unused_javascript: json["audits"]["unused-javascript"]["numericValue"].as_f64().unwrap_or(0.0),
        unused_css: json["audits"]["unused-css"]["numericValue"].as_f64().unwrap_or(0.0),
        dom_size: json["audits"]["dom-size"]["numericValue"].as_f64().unwrap_or(0.0),
        preconnect_origins: json["audits"]["preconnect-to-required-origins"]["numericValue"].as_f64().unwrap_or(0.0),
        properly_sized_images: json["audits"]["uses-responsive-images"]["numericValue"].as_f64().unwrap_or(0.0),
        efficiently_encoded_images: json["audits"]["uses-optimized-images"]["numericValue"].as_f64().unwrap_or(0.0),
        minimize_main_thread_work: json["audits"]["mainthread-work-breakdown"]["numericValue"].as_f64().unwrap_or(0.0),
        minimize_render_blocking_stylesheets: json["audits"]["uses-rel-preload"]["numericValue"].as_f64().unwrap_or(0.0),
        avoid_large_layout_shifts: json["audits"]["layout-shift-elements"]["numericValue"].as_f64().unwrap_or(0.0),
    }
}
