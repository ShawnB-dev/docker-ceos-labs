use tokio::{
    fs::{File, metadata},
    io::{AsyncBufReadExt, BufReader, AsyncSeekExt},
    time::{sleep, Duration},
};
use tokio_stream::wrappers::LinesStream;
use tokio_stream::StreamExt;
use regex::Regex;
use anyhow::{Result, Context};
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() -> Result<()> {
    println!("📊 Starting Rust Log Parser Application...");

    let log_file_path = "/var/log/syslog/remote-logs.log";

    // Regex to parse the syslog-ng template: "${ISODATE} ${HOST} ${MSGHDR}${MSG}\n"
    // Example: "2026-05-16T15:30:00.123456+00:00 spine-1 %SYS-5-CONFIG_I: Configured from console by admin on console"
    let log_regex = Regex::new(
        r"^(?P<timestamp>\S+) (?P<host>\S+) (?P<message_header>%?\S+-\d+-\S+): (?P<message>.*)$"
    ).context("Failed to compile log regex")?;

    loop {
        match File::open(log_file_path).await {
            Ok(file) => {
                println!("✅ Monitoring log file: {}", log_file_path);
                // Simple and efficient way to seek to the end of the file at startup
                let mut current_pos = metadata(log_file_path).await.map(|m| m.len()).unwrap_or(0);

                loop {
                    // Re-open file to check for new content (handles log rotation/truncation)
                    let new_file = File::open(log_file_path).await?;
                    let mut new_reader = BufReader::new(new_file);
                    new_reader.seek(tokio::io::SeekFrom::Start(current_pos)).await?;
                    let mut new_lines = LinesStream::new(new_reader.lines());

                    while let Some(line_result) = new_lines.next().await {
                        let line = line_result.context("Failed to read line from log file")?;
                        current_pos += line.len() as u64 + 1; // +1 for newline character

                        if let Some(captures) = log_regex.captures(&line) {
                            let timestamp_str = captures.name("timestamp").map_or("", |m| m.as_str());
                            let host = captures.name("host").map_or("", |m| m.as_str());
                            let message_header = captures.name("message_header").map_or("", |m| m.as_str());
                            let message = captures.name("message").map_or("", |m| m.as_str());

                            let parsed_timestamp = DateTime::parse_from_rfc3339(timestamp_str)
                                .map(|dt| dt.with_timezone(&Utc))
                                .unwrap_or_else(|_| Utc::now());

                            println!("[{}] Host: {}, Type: {}, Message: {}", parsed_timestamp, host, message_header, message);
                        } else {
                            println!("Unparsed log: {}", line);
                        }
                    }
                    sleep(Duration::from_secs(1)).await; // Poll for new logs every second
                }
            }
            Err(e) => {
                eprintln!("❌ Error opening log file {}: {}. Retrying in 5s...", log_file_path, e);
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}