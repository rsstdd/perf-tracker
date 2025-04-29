# Performance Tracker

Performance Tracker is a Rust-based tool designed to measure and track the performance of web applications using Lighthouse. This tool fetches key performance metrics for a given URL and evaluates them based on predefined criteria. The metrics are logged to a file for further analysis.

## Features

- Fetches performance metrics using Lighthouse
- Converts metrics to seconds for readability
- Evaluates metrics and provides a summary (Good, Needs Improvement, Poor)
- Logs metrics and summaries to a file with the current date
- Supports running multiple tests and calculating averages

## Prerequisites

- Rust and Cargo installed
- Lighthouse installed globally (`npm install -g lighthouse`)
- Docker and PostgreSQL for database logging (optional, if database logging is enabled)

## Setup

1. **Clone the repository:**

```sh
git clone https://github.com/yourusername/performance-tracker.git
cd performance-tracker
```

2. **Install dependencies:**

```sh
cargo build
```

3. **Set up environment variables:**

Create a `.env` file in the project root directory and add the following environment variables:

```sh
DATABASE_URL=your_database_url
```

## Usage

### Running the Tracker

1. **Run the tracker:**

```sh
cargo run
```

This will fetch the performance metrics for `https://alaskaair.com` and log the results to a file named `metrics_log_<date>.txt`.

### Modifying the URL

To fetch metrics for a different URL, modify the `url` variable in the `main` function:

```rust
let url = "https://yourwebsite.com";
```

## Project Structure

- `src/main.rs`: Main application logic and functions for fetching and saving metrics.
- `.env`: Environment variables file for database connection.

## Acknowledgments

- [Lighthouse](https://github.com/GoogleChrome/lighthouse) - Automated auditing, performance metrics, and best practices for the web.
- [dotenv](https://github.com/dotenv-rs/dotenv) - Load environment variables from `.env` files.
- [tokio](https://tokio.rs/) - Asynchronous runtime for Rust.
- [serde](https://serde.rs/) - Serialization framework for Rust.
- [chrono](https://docs.rs/chrono/) - Date and time library for Rust.

Here is the fully updated README.md, reflecting your current modularized architecture, trace file integration, summary reporting, and Lighthouse scenario runner:

⸻

Performance Tracker

Performance Tracker is a modular, extensible Rust-based CLI tool that evaluates the runtime performance of web applications using Google’s Lighthouse. It automates scenario testing, averages results across runs, stores logs, generates cumulative reports, and optionally parses .trace.json files for CPU task duration insights.

🚀 Features

- 📊 Fetches detailed Lighthouse performance audits for each scenario
- ♻️ Averages multiple runs per scenario to ensure measurement stability
- 🧠 Classifies metrics into “Good”, “Needs Improvement”, or “Poor”
- 📝 Saves results as human-readable logs and cumulative summary.json
- 🧵 Parses .trace.json files to surface top main thread bottlenecks
- 🧩 Easily extendable via modular Rust crate architecture
- 🧪 Designed to benchmark blocking third-party scripts (Tealium, Optimizely, etc.)

⸻

📦 Project Structure

src/
├── main.rs                      # Entrypoint with scenario orchestrator
├── metrics.rs                   # Core LighthouseMetrics struct and analysis
├── lighthouse.rs                # Lighthouse fetch logic and scenario CLI runner
├── trace.rs                     # Trace file analysis and CPU bottleneck detection
├── summary.rs                   # Cumulative summary writer for summary.json
└── lighthouse_summary.rs        # Optional post-run visual or logging hooks

⸻

🛠 Prerequisites

- Rust + Cargo
- Lighthouse CLI installed globally:

`npm install -g lighthouse`

- (Optional) PostgreSQL or SQLite if you plan to persist results

⸻

⚙️ Setup

 1. Clone the repo:

```sh
git clone <https://github.com/yourusername/performance-tracker.git>
cd performance-tracker
```

 2. Build dependencies:

```sh
cargo build --release
```

 3. Create a .env file (optional for DB):

```sh
echo "DATABASE_URL=postgres://user:password@localhost/dbname" > .env
```

⸻

▶️ Usage

Run All Scenarios

Execute the full scenario suite (e.g. blocking Tealium, Optimizely, etc.):

```sh
cargo run --release
```

Each scenario will:

- Execute lighthouse 3 times
- Average the results
- Log human-readable metrics to `metrics_log_<date>.txt`
- Append JSON to summary.json
- Print top 5 performance bottlenecks
- Confirm `.trace.json` file creation and parse it for main thread duration peaks

Customize Test Targets

Update the scenarios list in main.rs to include or modify tested conditions:

("no-optimizely", "<https://example.com>", vec!["*.optimizely.com"])

⸻

🧪 Sample Output

```sh
=== Running Scenario: no-tealium ===
-> Run 1/3 ...
✅ Trace JSON generated: lighthouse_report_no-tealium_2025-04-29.json

Summary for scenario 'no-tealium':

- Time to Interactive: 2.45s – Good
- Total Blocking Time: 0.21s – Needs Improvement
...

Top 5 Performance Bottlenecks:

- Total Blocking Time: 0.21
- Render Blocking Resources: 0.15
...

Top 5 Main Thread Task Durations (ms):

- 132.00 ms
- 89.60 ms
```

⸻

📈 Trace Analysis

The tool parses Lighthouse-generated `.trace.json` files, extracting RunTask events to identify CPU bottlenecks.

Confirm that --save-assets is enabled in Lighthouse CLI args to persist `.trace.json`.

⸻

📘 Acknowledgments

- Google Lighthouse
- Tokio
- Serde
- Chrono
- dotenv

⸻
