use dotenv::dotenv;
// use std::env;
use std::error::Error;
// use tokio_postgres::NoTls;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde::{Serialize, Deserialize};
use chrono::{Utc, Local};
use std::process::Command;
use serde_json::{Value, to_string_pretty};


#[derive(Debug, Serialize, Deserialize)]
struct LighthouseMetrics {
    first_contentful_paint: f64,
    largest_contentful_paint: f64,
    time_to_interactive: f64,
    total_blocking_time: f64,
    cumulative_layout_shift: f64,
    speed_index: f64,
}

async fn save_metrics_to_db(metrics: &LighthouseMetrics, url: &str, fetch_time: &str) -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    // let database_url = env::var("DATABASE_URL")?;
    // let (client, connection) = tokio_postgres::connect(&database_url, NoTls).await?;

    // tokio::spawn(async move {
    //     if let Err(e) = connection.await {
    //         eprintln!("connection error: {}", e);
    //     }
    // });

    // // Ensure fetch_time is not empty or null
    // if fetch_time.is_empty() {
    //     return Err(Box::from("fetch_time is missing"));
    // }

    // // Create table if it doesn't exist
    // client.execute(
    //     "
    //     CREATE TABLE IF NOT EXISTS metrics (
    //         id SERIAL PRIMARY KEY,
    //         url TEXT NOT NULL,
    //         fetch_time TIMESTAMPTZ NOT NULL,
    //         first_contentful_paint REAL NOT NULL,
    //         largest_contentful_paint REAL NOT NULL,
    //         time_to_interactive REAL NOT NULL,
    //         total_blocking_time REAL NOT NULL,
    //         cumulative_layout_shift REAL NOT NULL,
    //         speed_index REAL NOT NULL
    //     );
    //     ",
    //     &[],
    // ).await?;

    // // Insert metrics into the database
    // client.execute(
    //     "
    //     INSERT INTO metrics (
    //         url, fetch_time, first_contentful_paint, largest_contentful_paint,
    //         time_to_interactive, total_blocking_time, cumulative_layout_shift, speed_index
    //     ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8);
    //     ",
    //     &[
    //         &url,
    //         &fetch_time,
    //         &metrics.first_contentful_paint,
    //         &metrics.largest_contentful_paint,
    //         &metrics.time_to_interactive,
    //         &metrics.total_blocking_time,
    //         &metrics.cumulative_layout_shift,
    //         &metrics.speed_index,
    //     ],
    // ).await?;

    // Ensure fetch_time is not empty or null
    if fetch_time.is_empty() {
        return Err(Box::from("fetch_time is missing"));
    }

    let current_date = Local::now().format("%Y-%m-%d").to_string();
    let file_name = format!("metrics_log_{}.txt", current_date);
    let mut file = File::create(file_name).await?;
    let data = format!(
        "URL: {}\nFetch Time: {}\nFirst Contentful Paint: {}\nLargest Contentful Paint: {}\nTime to Interactive: {}\nTotal Blocking Time: {}\nCumulative Layout Shift: {}\nSpeed Index: {}\n\n",
        url, fetch_time, metrics.first_contentful_paint, metrics.largest_contentful_paint, metrics.time_to_interactive, metrics.total_blocking_time, metrics.cumulative_layout_shift, metrics.speed_index
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
    let metrics = fetch_lighthouse_metrics(url).await?;
    let fetch_time = Utc::now().to_rfc3339();

    save_metrics_to_db(&metrics, url, &fetch_time).await?;

    Ok(())
}
