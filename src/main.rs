mod banner;
mod args;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;
use reqwest::{StatusCode};
use std::fs::File;
use std::io::{BufRead, BufReader};
use clap::Parser;
use tokio::time::Instant;
use termion::{color};

struct FileResult {
    path: String,
    status: StatusCode,
    extension: String
}

fn get_line_count(file_path: &str) -> usize {
    let input = File::open(file_path).expect("Cannot open wordlist");
    let buffered = BufReader::new(input);
    let line_count = buffered.lines().count();
    line_count
}

fn main() {
    let args = args::Args::parse();

    println!("{}{}", color::Fg(color::LightMagenta), banner::banner());

    let threads = usize::from(args.threads);

    let line_count = get_line_count(&args.wordlist);
    let extension_count = &args.extension.len();

    let reader = BufReader::new(File::open(&args.wordlist).expect("Cannot open wordlist"));

    let pool = ThreadPool::new(threads);
    let (tx, rx) = channel();

    let now = Instant::now();
    for line in reader.lines() {
        let l = line.unwrap();
        for extension in &args.extension {
            let tx = tx.clone();
            let url = args.url.to_owned().clone();
            let ext = extension.to_owned().clone();
            let m = l.clone();
            pool.execute(move|| {
                let url = format!("{}{}{}.{}", url, "/", m, ext);
                let status = reqwest::blocking::get(&url).unwrap().status();
                let res = FileResult {
                    status,
                    path: m.to_string(),
                    extension: ext.to_string()
                };

                tx.send(res).expect("channel will be there waiting for the pool");
            });
        }
    }

    for result in rx.iter().take(line_count * extension_count) {
        println!("{}.{} [{}]", result.path, result.extension, result.status);
    }

    println!("Done. [{} milliseconds]", now.elapsed().as_millis());
}
