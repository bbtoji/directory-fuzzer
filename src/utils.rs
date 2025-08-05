use indicatif::{self, ProgressBar, ProgressStyle};
use reqwest::StatusCode;
use std::collections::HashSet;
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use url::Url;
use colored::Colorize;

pub async fn parse_args(
    url: String,
    wordlist: String,
    ignore: String,
) -> std::io::Result<(Url, Vec<String>, HashSet<StatusCode>)> {
    let words = read_wordlist(wordlist).await?;
    let url = Url::parse(&url).expect("[!] Parse Error!");
    let ignored_status_codes = read_ignore_status_code(&ignore);

    Ok((url, words, ignored_status_codes))
}

async fn read_wordlist(file: String) -> std::io::Result<Vec<String>> {
    let filepath = Path::new(&file);
    let file = File::open(filepath).await?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut words = Vec::new();

    while let Some(line) = lines.next_line().await? {
        words.push(line);
    }

    Ok(words)
}

fn read_ignore_status_code(ignore: &str) -> HashSet<StatusCode> {
    ignore
        .split(",")
        .filter_map(|x| {
            let code = x.parse::<u16>().ok()?;
            StatusCode::from_u16(code).ok()
        })
        .collect()
}

pub fn get_progress_bar(total: u64) -> ProgressBar {
    let pb = ProgressBar::new(total);
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.white} {msg}")
            .unwrap()
            .tick_strings(&[
                format!("[{}]", "⣀".green()).as_str(),
                format!("[{}]", "⣄".green()).as_str(),
                format!("[{}]", "⣤".green()).as_str(),
                format!("[{}]", "⣦".green()).as_str(),
                format!("[{}]", "⣶".green()).as_str(),
                format!("[{}]", "⣷".green()).as_str(),
                format!("[{}]", "⣿".green()).as_str(),
            ]),
    );
    pb.set_message("Fuzzing...");
    pb
}
