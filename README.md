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
