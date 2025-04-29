use serde::{Deserialize, Serialize};
use std::error::Error;
use std::process::Command;
use chrono::Local;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use serde_json::{to_string_pretty, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LighthouseMetrics {
    pub first_contentful_paint: f64,
    pub largest_contentful_paint: f64,
    pub time_to_interactive: f64,
    pub total_blocking_time: f64,
    pub cumulative_layout_shift: f64,
    pub speed_index: f64,
    pub performance_score: f64,
    pub first_meaningful_paint: f64,
    pub first_cpu_idle: f64,
    pub max_potential_fid: f64,
    pub estimated_input_latency: f64,
    pub server_response_time: f64,
    pub javascript_bootup_time: f64,
    pub total_byte_weight: f64,
    pub render_blocking_resources: f64,
    pub unused_javascript: f64,
    pub unused_css: f64,
    pub dom_size: f64,
    pub preconnect_origins: f64,
    pub properly_sized_images: f64,
    pub efficiently_encoded_images: f64,
    pub minimize_main_thread_work: f64,
    pub minimize_render_blocking_stylesheets: f64,
    pub avoid_large_layout_shifts: f64,
}

impl LighthouseMetrics {
    pub fn add(&mut self, other: &Self) {
        macro_rules! add_field {
            ($field:ident) => {
                self.$field += other.$field;
            };
        }
        add_field!(first_contentful_paint);
        add_field!(largest_contentful_paint);
        add_field!(time_to_interactive);
        add_field!(total_blocking_time);
        add_field!(cumulative_layout_shift);
        add_field!(speed_index);
        add_field!(performance_score);
        add_field!(first_meaningful_paint);
        add_field!(first_cpu_idle);
        add_field!(max_potential_fid);
        add_field!(estimated_input_latency);
        add_field!(server_response_time);
        add_field!(javascript_bootup_time);
        add_field!(total_byte_weight);
        add_field!(render_blocking_resources);
        add_field!(unused_javascript);
        add_field!(unused_css);
        add_field!(dom_size);
        add_field!(preconnect_origins);
        add_field!(properly_sized_images);
        add_field!(efficiently_encoded_images);
        add_field!(minimize_main_thread_work);
        add_field!(minimize_render_blocking_stylesheets);
        add_field!(avoid_large_layout_shifts);
    }

    pub fn average(&mut self, count: f64) {
        macro_rules! div_field {
            ($field:ident) => {
                self.$field /= count;
            };
        }
        div_field!(first_contentful_paint);
        div_field!(largest_contentful_paint);
        div_field!(time_to_interactive);
        div_field!(total_blocking_time);
        div_field!(cumulative_layout_shift);
        div_field!(speed_index);
        div_field!(performance_score);
        div_field!(first_meaningful_paint);
        div_field!(first_cpu_idle);
        div_field!(max_potential_fid);
        div_field!(estimated_input_latency);
        div_field!(server_response_time);
        div_field!(javascript_bootup_time);
        div_field!(total_byte_weight);
        div_field!(render_blocking_resources);
        div_field!(unused_javascript);
        div_field!(unused_css);
        div_field!(dom_size);
        div_field!(preconnect_origins);
        div_field!(properly_sized_images);
        div_field!(efficiently_encoded_images);
        div_field!(minimize_main_thread_work);
        div_field!(minimize_render_blocking_stylesheets);
        div_field!(avoid_large_layout_shifts);
    }

    pub fn to_seconds(&self) -> Self {
        let mut clone = self.clone();
        macro_rules! to_sec {
            ($field:ident) => {
                clone.$field /= 1000.0;
            };
        }
        to_sec!(first_contentful_paint);
        to_sec!(largest_contentful_paint);
        to_sec!(time_to_interactive);
        to_sec!(total_blocking_time);
        to_sec!(speed_index);
        to_sec!(first_meaningful_paint);
        to_sec!(first_cpu_idle);
        to_sec!(max_potential_fid);
        to_sec!(estimated_input_latency);
        to_sec!(server_response_time);
        to_sec!(javascript_bootup_time);
        to_sec!(minimize_main_thread_work);
        to_sec!(minimize_render_blocking_stylesheets);
        clone
    }

    pub fn evaluate(&self) -> String {
        format!(
            "Performance Score: {:.2}\nFCP: {:.2}s\nLCP: {:.2}s\nTTI: {:.2}s\nTBT: {:.2}s",
            self.performance_score,
            self.first_contentful_paint,
            self.largest_contentful_paint,
            self.time_to_interactive,
            self.total_blocking_time
        )
    }

    pub fn top_offenders(&self) -> Vec<(&'static str, f64)> {
        let mut offenders = vec![
            ("TBT", self.total_blocking_time),
            ("TTI", self.time_to_interactive),
            ("JS Bootup", self.javascript_bootup_time),
            ("DOM Size", self.dom_size),
            ("Byte Weight", self.total_byte_weight),
        ];
        offenders.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        offenders
    }
}

pub async fn fetch_lighthouse_metrics(label: &str, url: &str, blocked: &[&str]) -> Result<LighthouseMetrics, Box<dyn Error>> {
    let mut args = vec![
        url,
        "--output=json",
        "--output-path=stdout",
        "--quiet",
        "--preset=desktop",
        "--only-categories=performance",
        "--save-assets",
        "--headless",
    ];

    for pattern in blocked {
        args.push("--blocked-url-patterns");
        args.push(pattern);
    }

    let output = Command::new("lighthouse").args(&args).output()?;
    if !output.status.success() {
        return Err(format!("Lighthouse failed: {}", output.status).into());
    }

    let stdout = String::from_utf8(output.stdout)?;
    let json: Value = serde_json::from_str(&stdout)?;

    let filename = format!("lighthouse_report_{}_{}.json", label, Local::now().format("%Y-%m-%d"));
    File::create(&filename).await?.write_all(to_string_pretty(&json)?.as_bytes()).await?;

    let metrics = LighthouseMetrics {
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
    };

    Ok(metrics)
}
