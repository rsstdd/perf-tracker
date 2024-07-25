use dotenv::dotenv;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde::{Serialize, Deserialize};
use chrono::{Utc, Local};
use std::process::Command;
use serde_json::{Value, to_string_pretty};

#[derive(Debug, Serialize, Deserialize, Default)]
struct LighthouseMetrics {
    first_contentful_paint: f64,
    largest_contentful_paint: f64,
    time_to_interactive: f64,
    total_blocking_time: f64,
    cumulative_layout_shift: f64,
    speed_index: f64,
}

impl LighthouseMetrics {
    fn add(&mut self, other: &LighthouseMetrics) {
        self.first_contentful_paint += other.first_contentful_paint;
        self.largest_contentful_paint += other.largest_contentful_paint;
        self.time_to_interactive += other.time_to_interactive;
        self.total_blocking_time += other.total_blocking_time;
        self.cumulative_layout_shift += other.cumulative_layout_shift;
        self.speed_index += other.speed_index;
    }

    fn average(&mut self, count: f64) {
        self.first_contentful_paint /= count;
        self.largest_contentful_paint /= count;
        self.time_to_interactive /= count;
        self.total_blocking_time /= count;
        self.cumulative_layout_shift /= count;
        self.speed_index /= count;
    }

    fn to_seconds(&self) -> Self {
        Self {
            first_contentful_paint: self.first_contentful_paint / 1000.0,
            largest_contentful_paint: self.largest_contentful_paint / 1000.0,
            time_to_interactive: self.time_to_interactive / 1000.0,
            total_blocking_time: self.total_blocking_time / 1000.0,
            cumulative_layout_shift: self.cumulative_layout_shift,
            speed_index: self.speed_index / 1000.0,
        }
    }

    fn evaluate(&self) -> String {
        let fcp_eval = match self.first_contentful_paint {
            x if x <= 1.8 => "Good",
            x if x <= 3.0 => "Needs Improvement",
            _ => "Poor",
        };

        let lcp_eval = match self.largest_contentful_paint {
            x if x <= 2.5 => "Good",
            x if x <= 4.0 => "Needs Improvement",
            _ => "Poor",
        };

        let tti_eval = match self.time_to_interactive {
            x if x <= 3.8 => "Good",
            x if x <= 7.3 => "Needs Improvement",
            _ => "Poor",
        };

        let tbt_eval = match self.total_blocking_time {
            x if x <= 0.2 => "Good",
            x if x <= 0.6 => "Needs Improvement",
            _ => "Poor",
        };

        let cls_eval = match self.cumulative_layout_shift {
            x if x <= 0.1 => "Good",
            x if x <= 0.25 => "Needs Improvement",
            _ => "Poor",
        };

        let si_eval = match self.speed_index {
            x if x <= 3.4 => "Good",
            x if x <= 5.8 => "Needs Improvement",
            _ => "Poor",
        };

        format!(
            "First Contentful Paint: {:.2} seconds - {}\n\
            Largest Contentful Paint: {:.2} seconds - {}\n\
            Time to Interactive: {:.2} seconds - {}\n\
            Total Blocking Time: {:.3} seconds - {}\n\
            Cumulative Layout Shift: {:.3} - {}\n\
            Speed Index: {:.2} seconds - {}\n",
            self.first_contentful_paint, fcp_eval,
            self.largest_contentful_paint, lcp_eval,
            self.time_to_interactive, tti_eval,
            self.total_blocking_time, tbt_eval,
            self.cumulative_layout_shift, cls_eval,
            self.speed_index, si_eval,
        )
    }
}

async fn save_metrics_to_db(metrics: &LighthouseMetrics, url: &str, fetch_time: &str) -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    if fetch_time.is_empty() {
        return Err(Box::from("fetch_time is missing"));
    }

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("metrics_log_{}.txt", current_date);
    let mut file = File::create(file_name).await?;
    let summary = metrics.evaluate();
    let data = format!(
        "URL: {}\nFetch Time: {}\nFirst Contentful Paint: {:.2} seconds\nLargest Contentful Paint: {:.2} seconds\nTime to Interactive: {:.2} seconds\nTotal Blocking Time: {:.3} seconds\nCumulative Layout Shift: {:.3}\nSpeed Index: {:.2} seconds\n\nSummary:\n{}",
        url, fetch_time, metrics.first_contentful_paint, metrics.largest_contentful_paint, metrics.time_to_interactive, metrics.total_blocking_time, metrics.cumulative_layout_shift, metrics.speed_index, summary
    );
    file.write_all(data.as_bytes()).await?;

    Ok(())
}

async fn fetch_lighthouse_metrics(url: &str) -> Result<LighthouseMetrics, Box<dyn Error>> {
    let output = Command::new("lighthouse")
        .args(&[url, "--output=json", "--quiet", "--view", "--window-size=100,000,000", "--preset=desktop", "--headless", "--only-categories=performance,seo"])
        .output()?;

    if !output.status.success() {
        return Err(format!(
            "Lighthouse command failed with status: {}",
            output.status
        )
        .into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let json: serde_json::Value = serde_json::from_str(&stdout)?;

    let formatted_json = to_string_pretty(&json)?;

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("metrics_log_{}.json", current_date);
    let mut file = File::create(file_name).await?;
    file.write_all(formatted_json.as_bytes()).await?;

    println!("{}", formatted_json);

    let metrics = LighthouseMetrics {
        first_contentful_paint: json["audits"]["first-contentful-paint"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        largest_contentful_paint: json["audits"]["largest-contentful-paint"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        time_to_interactive: json["audits"]["interactive"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        total_blocking_time: json["audits"]["total-blocking-time"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        cumulative_layout_shift: json["audits"]["cumulative-layout-shift"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        speed_index: json["audits"]["speed-index"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
    };

    Ok(metrics)
}

// alaskaair usage
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://alaskaair.com";
    let mut total_metrics = LighthouseMetrics::default();
    let mut successful_runs = 0;

    for _ in 0..5 {
        match fetch_lighthouse_metrics(url).await {
            Ok(metrics) => {
                total_metrics.add(&metrics);
                successful_runs += 1;
            },
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    if successful_runs > 0 {
        total_metrics.average(successful_runs as f64);
        let metrics_in_seconds = total_metrics.to_seconds();
        let fetch_time = Utc::now().to_rfc3339();
        save_metrics_to_db(&metrics_in_seconds, url, &fetch_time).await?;
    } else {
        eprintln!("All attempts to fetch Lighthouse metrics failed.");
    }

    Ok(())
}
