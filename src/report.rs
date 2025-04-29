use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use chrono::Local;

use crate::metrics::LighthouseMetrics;

pub async fn save_metrics_to_db(metrics: &LighthouseMetrics, url: &str, time: &str) -> Result<(), Box<dyn Error>> {
    let filename = format!("metrics_log_{}.txt", Local::now().format("%Y-%m-%d"));
    let mut file = File::create(filename).await?;
    let summary = metrics.evaluate();
    let data = format!("{}\nFetch Time: {}\n{}\n", url, time, summary);
    file.write_all(data.as_bytes()).await?;
    Ok(())
}

/// Save a plain-text version of the metrics for human inspection.
pub async fn save_metrics_to_txt(
    metrics: &LighthouseMetrics,
    url: &str,
    fetch_time: &str,
) -> Result<(), Box<dyn Error>> {
    let date = Local::now().format("%Y-%m-%d").to_string();
    let filename = format!("metrics_log_{}.txt", date);
    let mut file = tokio::fs::File::create(filename).await?;
    let summary = metrics.evaluate();
    let content = format!("URL: {}\nFetch Time: {}\n{}\n", url, fetch_time, summary);
    file.write_all(content.as_bytes()).await?;
    Ok(())
}
