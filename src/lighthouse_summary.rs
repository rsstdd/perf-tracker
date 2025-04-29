use dotenv::dotenv;
use std::error::Error;
use std::fs;
use std::process::Command;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde::{Serialize, Deserialize};
use chrono::Local;
use serde_json::to_string_pretty;

#[derive(Debug, Serialize, Deserialize, Default)]
struct LighthouseMetrics {
    first_contentful_paint: f64,
    largest_contentful_paint: f64,
    time_to_interactive: f64,
    total_blocking_time: f64,
    cumulative_layout_shift: f64,
    speed_index: f64,
    performance_score: f64,
    first_meaningful_paint: f64,
    first_cpu_idle: f64,
    max_potential_fid: f64,
    estimated_input_latency: f64,
    server_response_time: f64,
    javascript_bootup_time: f64,
    total_byte_weight: f64,
    render_blocking_resources: f64,
    unused_javascript: f64,
    unused_css: f64,
    dom_size: f64,
    preconnect_origins: f64,
    properly_sized_images: f64,
    efficiently_encoded_images: f64,
    minimize_main_thread_work: f64,
    minimize_render_blocking_stylesheets: f64,
    avoid_large_layout_shifts: f64,
}

struct ScenarioMetrics {
    name: String,
    perf_score: f64,
    fcp: f64,
    lcp: f64,
    tti: f64,
    tbt: f64,
    delta_perf: f64,
}




async fn fetch_lighthouse_metrics(label: &str, url: &str, blocked_patterns: &[&str]) -> Result<LighthouseMetrics, Box<dyn Error>> {
    let mut args = vec![
        url,
        "--output=json",
        "--output-path=stdout",
        "--quiet",
        "--window-size=1000,1000",
        "--preset=desktop",
        "--headless",
        "--only-categories=performance,accessibility,seo,best-practices",
    ];

    for pattern in blocked_patterns {
        args.push("--blocked-url-patterns");
        args.push(pattern);
    }

    let output = Command::new("lighthouse")
        .args(&args)
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Lighthouse command failed with status: {}",
            output.status
        ).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let json: serde_json::Value = serde_json::from_str(&stdout)?;
    let formatted_json = to_string_pretty(&json)?;

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("lighthouse_report_{}_{}.json", label, current_date);
    let mut file = File::create(&file_name).await?;
    file.write_all(formatted_json.as_bytes()).await?;

    println!("Saved full report for scenario '{}'", label);

    Ok(LighthouseMetrics {
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
    })
}

pub async fn run_lighthouse_scenarios() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    let scenarios = vec![
        ("baseline", "https://alaskaair.com", vec![]),
        ("no-tealium", "https://alaskaair.com", vec!["*.tealiumiq.com"]),
        ("no-appd", "https://alaskaair.com", vec!["*.appdynamics.com"]),
        ("no-optimizely", "https://alaskaair.com", vec!["*.optimizely.com"]),
        ("no-header-footer", "https://alaskaair.com", vec!["*/header*", "*/footer*"]),
        ("no-quantum", "https://alaskaair.com", vec!["*.quantummetric.com"]),
    ];

    let date = Local::now().format("%Y-%m-%d").to_string();
    let mut summary_data = Vec::new();
    let mut baseline_score = 0.0;

    for (label, url, blocked) in scenarios {
        println!("\n=== Running Scenario: {} ===", label);

        let metrics = fetch_lighthouse_metrics(label, url, &blocked).await?;

        if label == "baseline" {
            baseline_score = metrics.performance_score;
        }

        summary_data.push(ScenarioMetrics {
            name: label.to_string(),
            perf_score: metrics.performance_score,
            fcp: metrics.first_contentful_paint / 1000.0,
            lcp: metrics.largest_contentful_paint / 1000.0,
            tti: metrics.time_to_interactive / 1000.0,
            tbt: metrics.total_blocking_time / 1000.0,
            delta_perf: 0.0,
        });
    }

    for item in &mut summary_data {
        item.delta_perf = item.perf_score - baseline_score;
    }

    summary_data.sort_by(|a, b| b.delta_perf.partial_cmp(&a.delta_perf).unwrap_or(std::cmp::Ordering::Equal));

    let mut markdown = String::new();
    markdown.push_str("# Lighthouse Performance Summary\n\n");
    markdown.push_str("| Scenario           | Perf | ΔPerf | FCP   | LCP   | TTI   | TBT  |\n");
    markdown.push_str("|--------------------|------|-------|-------|-------|-------|------|\n");

    for s in &summary_data {
        markdown.push_str(&format!(
            "| {:<18} | {:>4.1} | {:>+6.1} | {:>4.2}s | {:>4.2}s | {:>4.2}s | {:>4.2}s |\n",
            s.name, s.perf_score, s.delta_perf, s.fcp, s.lcp, s.tti, s.tbt
        ));
    }

    let summary_filename = format!("summary_{}.md", date);
    fs::write(&summary_filename, markdown)?;
    println!("Markdown summary written to {}", summary_filename);

    Ok(())
}
