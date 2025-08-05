use clap::Parser;
use colored::Colorize;

use crate::bruteforce::Bruteforcer;

mod bruteforce;
mod utils;

#[derive(Parser, Debug)]
#[command(author = "bbtoji", about = "Rust Port Scanner")]
struct Args {
    #[arg(short, long)]
    url: String,

    #[arg(short, long)]
    wordlist: String,

    // List of status codes to ignore (e.g. 404,302,201)
    #[arg(short, long, default_value = "404")]
    ignore: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let (url, words, ignore) = utils::parse_args(args.url, args.wordlist, args.ignore)
        .await
        .unwrap();

    let dot = "*".green();

    println!("\n====================================");
    println!("[{}] Scheme: {:?}", dot, url.scheme());
    if let Some(host) = url.host() {
        println!("[{}] Host: {}", dot, host);
    }
    println!("[{}] URI: {}", dot, url.path());
    println!("[{}] Ignored status codes: {:?}", dot, ignore);
    println!("\n====================================");

    let bruteforcer = Bruteforcer::new(url, words, ignore);
    bruteforcer.run().await;
}
