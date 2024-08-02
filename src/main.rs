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

impl LighthouseMetrics {
    fn add(&mut self, other: &LighthouseMetrics) {
        self.first_contentful_paint += other.first_contentful_paint;
        self.largest_contentful_paint += other.largest_contentful_paint;
        self.time_to_interactive += other.time_to_interactive;
        self.total_blocking_time += other.total_blocking_time;
        self.cumulative_layout_shift += other.cumulative_layout_shift;
        self.speed_index += other.speed_index;
        self.performance_score += other.performance_score;
        self.first_meaningful_paint += other.first_meaningful_paint;
        self.first_cpu_idle += other.first_cpu_idle;
        self.max_potential_fid += other.max_potential_fid;
        self.estimated_input_latency += other.estimated_input_latency;
        self.server_response_time += other.server_response_time;
        self.javascript_bootup_time += other.javascript_bootup_time;
        self.total_byte_weight += other.total_byte_weight;
        self.render_blocking_resources += other.render_blocking_resources;
        self.unused_javascript += other.unused_javascript;
        self.unused_css += other.unused_css;
        self.dom_size += other.dom_size;
        self.preconnect_origins += other.preconnect_origins;
        self.properly_sized_images += other.properly_sized_images;
        self.efficiently_encoded_images += other.efficiently_encoded_images;
        self.minimize_main_thread_work += other.minimize_main_thread_work;
        self.minimize_render_blocking_stylesheets += other.minimize_render_blocking_stylesheets;
        self.avoid_large_layout_shifts += other.avoid_large_layout_shifts;
    }

    fn average(&mut self, count: f64) {
        self.first_contentful_paint /= count;
        self.largest_contentful_paint /= count;
        self.time_to_interactive /= count;
        self.total_blocking_time /= count;
        self.cumulative_layout_shift /= count;
        self.speed_index /= count;
        self.performance_score /= count;
        self.first_meaningful_paint /= count;
        self.first_cpu_idle /= count;
        self.max_potential_fid /= count;
        self.estimated_input_latency /= count;
        self.server_response_time /= count;
        self.javascript_bootup_time /= count;
        self.total_byte_weight /= count;
        self.render_blocking_resources /= count;
        self.unused_javascript /= count;
        self.unused_css /= count;
        self.dom_size /= count;
        self.preconnect_origins /= count;
        self.properly_sized_images /= count;
        self.efficiently_encoded_images /= count;
        self.minimize_main_thread_work /= count;
        self.minimize_render_blocking_stylesheets /= count;
        self.avoid_large_layout_shifts /= count;
    }

    fn to_seconds(&self) -> Self {
        Self {
            first_contentful_paint: self.first_contentful_paint / 1000.0,
            largest_contentful_paint: self.largest_contentful_paint / 1000.0,
            time_to_interactive: self.time_to_interactive / 1000.0,
            total_blocking_time: self.total_blocking_time / 1000.0,
            cumulative_layout_shift: self.cumulative_layout_shift,
            speed_index: self.speed_index / 1000.0,
            performance_score: self.performance_score,
            first_meaningful_paint: self.first_meaningful_paint / 1000.0,
            first_cpu_idle: self.first_cpu_idle / 1000.0,
            max_potential_fid: self.max_potential_fid / 1000.0,
            estimated_input_latency: self.estimated_input_latency / 1000.0,
            server_response_time: self.server_response_time / 1000.0,
            javascript_bootup_time: self.javascript_bootup_time / 1000.0,
            total_byte_weight: self.total_byte_weight,
            render_blocking_resources: self.render_blocking_resources,
            unused_javascript: self.unused_javascript,
            unused_css: self.unused_css,
            dom_size: self.dom_size,
            preconnect_origins: self.preconnect_origins,
            properly_sized_images: self.properly_sized_images,
            efficiently_encoded_images: self.efficiently_encoded_images,
            minimize_main_thread_work: self.minimize_main_thread_work,
            minimize_render_blocking_stylesheets: self.minimize_render_blocking_stylesheets,
            avoid_large_layout_shifts: self.avoid_large_layout_shifts,
        }
    }

    fn evaluate(&self) -> String {
        let mut good_count = 0;
        let mut needs_improvement_count = 0;
        let mut poor_count = 0;

        let fcp_eval = match self.first_contentful_paint {
            x if x <= 1.8 => {
                good_count += 1;
                "Good"
            }
            x if x <= 3.0 => {
                needs_improvement_count += 1;
                "Needs Improvement"
            }
            _ => {
                poor_count += 1;
                "Poor"
            }
        };

        let lcp_eval = match self.largest_contentful_paint {
            x if x <= 2.5 => {
                good_count += 1;
                "Good"
            }
            x if x <= 4.0 => {
                needs_improvement_count += 1;
                "Needs Improvement"
            }
            _ => {
                poor_count += 1;
                "Poor"
            }
        };

        let tti_eval = match self.time_to_interactive {
            x if x <= 3.8 => {
                good_count += 1;
                "Good"
            }
            x if x <= 7.3 => {
                needs_improvement_count += 1;
                "Needs Improvement"
            }
            _ => {
                poor_count += 1;
                "Poor"
            }
        };

        let tbt_eval = match self.total_blocking_time {
            x if x <= 0.2 => {
                good_count += 1;
                "Good"
            }
            x if x <= 0.6 => {
                needs_improvement_count += 1;
                "Needs Improvement"
            }
            _ => {
                poor_count += 1;
                "Poor"
            }
        };

        let cls_eval = match self.cumulative_layout_shift {
            x if x <= 0.1 => {
                good_count += 1;
                "Good"
            }
            x if x <= 0.25 => {
                needs_improvement_count += 1;
                "Needs Improvement"
            }
            _ => {
                poor_count += 1;
                "Poor"
            }
        };

        let si_eval = match self.speed_index {
            x if x <= 3.4 => {
                good_count += 1;
                "Good"
            }
            x if x <= 5.8 => {
                needs_improvement_count += 1;
                "Needs Improvement"
            }
            _ => {
                poor_count += 1;
                "Poor"
            }
        };

        let ps_eval = match self.performance_score {
            x if x >= 90.0 => {
                good_count += 1;
                "Good"
            }
            x if x >= 50.0 => {
                needs_improvement_count += 1;
                "Needs Improvement"
            }
            _ => {
                poor_count += 1;
                "Poor"
            }
        };

        format!(
            "First Contentful Paint: {:.2} seconds - {}\n\
            Largest Contentful Paint: {:.2} seconds - {}\n\
            Time to Interactive: {:.2} seconds - {}\n\
            Total Blocking Time: {:.3} seconds - {}\n\
            Cumulative Layout Shift: {:.3} - {}\n\
            Speed Index: {:.2} seconds - {}\n\
            Performance Score: {:.2} - {}\n\
            First Meaningful Paint: {:.2} seconds\n\
            First CPU Idle: {:.2} seconds\n\
            Max Potential FID: {:.2} seconds\n\
            Estimated Input Latency: {:.2} seconds\n\
            Server Response Time: {:.2} seconds\n\
            JavaScript Bootup Time: {:.2} seconds\n\
            Total Byte Weight: {:.2} bytes\n\
            Render Blocking Resources: {:.2} seconds\n\
            Unused JavaScript: {:.2} bytes\n\
            Unused CSS: {:.2} bytes\n\
            DOM Size: {:.2}\n\
            Preconnect Origins: {:.2}\n\
            Properly Sized Images: {:.2}\n\
            Efficiently Encoded Images: {:.2}\n\
            Minimize Main Thread Work: {:.2} seconds\n\
            Minimize Render Blocking Stylesheets: {:.2} seconds\n\
            Avoid Large Layout Shifts: {:.2}\n\n\
            Evaluation Summary:\n\
            Good: {}\n\
            Needs Improvement: {}\n\
            Poor: {}\n",
            self.first_contentful_paint, fcp_eval,
            self.largest_contentful_paint, lcp_eval,
            self.time_to_interactive, tti_eval,
            self.total_blocking_time, tbt_eval,
            self.cumulative_layout_shift, cls_eval,
            self.speed_index, si_eval,
            self.performance_score, ps_eval,
            self.first_meaningful_paint,
            self.first_cpu_idle,
            self.max_potential_fid,
            self.estimated_input_latency,
            self.server_response_time,
            self.javascript_bootup_time,
            self.total_byte_weight,
            self.render_blocking_resources,
            self.unused_javascript,
            self.unused_css,
            self.dom_size,
            self.preconnect_origins,
            self.properly_sized_images,
            self.efficiently_encoded_images,
            self.minimize_main_thread_work,
            self.minimize_render_blocking_stylesheets,
            self.avoid_large_layout_shifts,
            good_count,
            needs_improvement_count,
            poor_count,
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
        "URL: {}\nFetch Time: {}\nFirst Contentful Paint: {:.2} seconds\nLargest Contentful Paint: {:.2} seconds\nTime to Interactive: {:.2} seconds\nTotal Blocking Time: {:.3} seconds\nCumulative Layout Shift: {:.3}\nSpeed Index: {:.2} seconds\nPerformance Score: {:.2}\nFirst Meaningful Paint: {:.2} seconds\nFirst CPU Idle: {:.2} seconds\nMax Potential FID: {:.2} seconds\nEstimated Input Latency: {:.2} seconds\nServer Response Time: {:.2} seconds\nJavaScript Bootup Time: {:.2} seconds\nTotal Byte Weight: {:.2} bytes\nRender Blocking Resources: {:.2} seconds\nUnused JavaScript: {:.2} bytes\nUnused CSS: {:.2} bytes\nDOM Size: {:.2}\nPreconnect Origins: {:.2}\nProperly Sized Images: {:.2}\nEfficiently Encoded Images: {:.2}\nMinimize Main Thread Work: {:.2} seconds\nMinimize Render Blocking Stylesheets: {:.2} seconds\nAvoid Large Layout Shifts: {:.2}\n\nSummary:\n{}",
        url, fetch_time, metrics.first_contentful_paint, metrics.largest_contentful_paint, metrics.time_to_interactive, metrics.total_blocking_time, metrics.cumulative_layout_shift, metrics.speed_index, metrics.performance_score, metrics.first_meaningful_paint, metrics.first_cpu_idle, metrics.max_potential_fid, metrics.estimated_input_latency, metrics.server_response_time, metrics.javascript_bootup_time, metrics.total_byte_weight, metrics.render_blocking_resources, metrics.unused_javascript, metrics.unused_css, metrics.dom_size, metrics.preconnect_origins, metrics.properly_sized_images, metrics.efficiently_encoded_images, metrics.minimize_main_thread_work, metrics.minimize_render_blocking_stylesheets, metrics.avoid_large_layout_shifts, summary
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
        performance_score: json["categories"]["performance"]["score"]
            .as_f64()
            .unwrap_or(0.0) * 100.0,
        first_meaningful_paint: json["audits"]["first-meaningful-paint"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        first_cpu_idle: json["audits"]["first-cpu-idle"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        max_potential_fid: json["audits"]["max-potential-fid"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        estimated_input_latency: json["audits"]["estimated-input-latency"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        server_response_time: json["audits"]["server-response-time"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        javascript_bootup_time: json["audits"]["bootup-time"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        total_byte_weight: json["audits"]["total-byte-weight"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        render_blocking_resources: json["audits"]["render-blocking-resources"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        unused_javascript: json["audits"]["unused-javascript"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        unused_css: json["audits"]["unused-css"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        dom_size: json["audits"]["dom-size"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        preconnect_origins: json["audits"]["preconnect-to-required-origins"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        properly_sized_images: json["audits"]["uses-responsive-images"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        efficiently_encoded_images: json["audits"]["uses-optimized-images"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        minimize_main_thread_work: json["audits"]["mainthread-work-breakdown"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        minimize_render_blocking_stylesheets: json["audits"]["uses-rel-preload"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
        avoid_large_layout_shifts: json["audits"]["layout-shift-elements"]["numericValue"]
            .as_f64()
            .unwrap_or(0.0),
    };

    Ok(metrics)
}

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
