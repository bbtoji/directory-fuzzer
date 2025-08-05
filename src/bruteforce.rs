
use futures::future::join_all;
use indicatif::{ProgressBar};
use reqwest::{Client, StatusCode};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::Semaphore;
use url::Url;

use crate::utils;

pub struct Bruteforcer {
    url: Url,
    words: Vec<String>,
    ignore: HashSet<StatusCode>,
    client: Client,
}

impl Bruteforcer {
    pub fn new(url: Url, words: Vec<String>, ignore: HashSet<StatusCode>) -> Bruteforcer {
        Bruteforcer {
            url: url,
            words: words,
            ignore: ignore,
            client: Client::new(),
        }
    }
    pub async fn run(&self) {
        let client = Arc::new(self.client.clone());
        let ignore = Arc::new(self.ignore.clone());
        let mut tasks = Vec::new();

        let size = self.words.len() as u64;
        let pb = Arc::new(utils::get_progress_bar(size));
        let max_concurrent = 50;
        let semaphore = Arc::new(Semaphore::new(max_concurrent));

        for word in self.words.clone() {
            let client = client.clone();
            let url = self.url.clone();
            let ignore = ignore.clone();
            let semaphore = semaphore.clone();
            let pb = pb.clone();

            let trying = tokio::spawn(Bruteforcer::check_path(client, url, word, ignore, semaphore, pb));

            tasks.push(trying);
        }
        join_all(tasks).await;
        pb.finish_with_message("Done");
    }

    async fn check_path(
        client: Arc<Client>,
        mut url: Url,
        word: String,
        ignore: Arc<HashSet<StatusCode>>,
        sem: Arc<Semaphore>,
        pb: Arc<ProgressBar>,
    ) {
        let _permit = sem.acquire_owned().await.unwrap();
        let current_path = url.path();
        let new_path = format!("{}/{}", current_path.trim_end_matches("/"), word);

        url.set_path(&new_path);

        match client.get(url).send().await {
            Ok(response) => {
                let code = response.status();
                if !ignore.contains(&code) {
                    println!("[{code}] - {new_path}");
                }
            }
            Err(e) => {
                println!("[!] Error... -> {e}");
            }
        }
        pb.inc(1);
    }
}

