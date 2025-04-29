use std::fs::OpenOptions;
use std::io::{self, Write};
use serde_json::{json, Value};

fn write_summary_entry(
    scenario: &str,
    url: &str,
    fetch_time: &str,
    metrics: &LighthouseMetrics,
) -> io::Result<()> {
    let summary_file = "summary.json";
    let mut entries = if std::path::Path::new(summary_file).exists() {
        let content = std::fs::read_to_string(summary_file)?;
        serde_json::from_str::<Vec<Value>>(&content).unwrap_or_default()
    } else {
        Vec::new()
    };

    let new_entry = json!({
        "scenario": scenario,
        "url": url,
        "fetch_time": fetch_time,
        "metrics": {
            "performance_score": metrics.performance_score,
            "first_contentful_paint": metrics.first_contentful_paint,
            "largest_contentful_paint": metrics.largest_contentful_paint,
            "time_to_interactive": metrics.time_to_interactive,
            "total_blocking_time": metrics.total_blocking_time,
            "cumulative_layout_shift": metrics.cumulative_layout_shift,
            "speed_index": metrics.speed_index,
            "first_meaningful_paint": metrics.first_meaningful_paint,
            "first_cpu_idle": metrics.first_cpu_idle,
            "max_potential_fid": metrics.max_potential_fid,
            "estimated_input_latency": metrics.estimated_input_latency,
            "server_response_time": metrics.server_response_time,
            "javascript_bootup_time": metrics.javascript_bootup_time,
            "total_byte_weight": metrics.total_byte_weight,
            "render_blocking_resources": metrics.render_blocking_resources,
            "unused_javascript": metrics.unused_javascript,
            "unused_css": metrics.unused_css,
            "dom_size": metrics.dom_size,
            "preconnect_origins": metrics.preconnect_origins,
            "properly_sized_images": metrics.properly_sized_images,
            "efficiently_encoded_images": metrics.efficiently_encoded_images,
            "minimize_main_thread_work": metrics.minimize_main_thread_work,
            "minimize_render_blocking_stylesheets": metrics.minimize_render_blocking_stylesheets,
            "avoid_large_layout_shifts": metrics.avoid_large_layout_shifts,
        }
    });

    entries.push(new_entry);

    let pretty = serde_json::to_string_pretty(&entries)?;
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(summary_file)?;
    file.write_all(pretty.as_bytes())?;

    Ok(())
}
