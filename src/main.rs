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
    extension: String,
    size: u64
}

fn main() {
    let args = args::Args::parse();
    let threads = usize::from(args.threads);
    let extension_count = &args.extension.len();
    let line_count = get_line_count(&args.wordlist);

    println!("{}{}", color::Fg(color::LightMagenta), banner::banner(&args.url, &threads, &args.wordlist));
    println!("{}", color::Fg(color::Reset));

    let reader = BufReader::new(File::open(&args.wordlist).expect("Cannot open wordlist"));
    let pool = ThreadPool::new(threads);
    let (tx, rx) = channel();

    let now = Instant::now();
    for line in reader.lines() {
        if let Ok(l) = line {
            for extension in &args.extension {
                let tx = tx.clone();
                let url = args.url.to_owned().clone();
                let ext = extension.to_owned().clone();
                let path = l.clone();
                pool.execute(move || {
                    let url = format!("{}{}{}.{}", url, "/", path, ext);

                    if let Ok(res) = reqwest::blocking::get(&url) {
                        let status = res.status();
                        let size = res.content_length();

                        let res = FileResult {
                            status,
                            path: path.to_string(),
                            extension: ext.to_string(),
                            size: size.unwrap_or(0)
                        };

                        tx.send(res).expect("channel will be there waiting for the pool");
                    }
                });
            }
        }
    }

    for result in rx.iter().take(line_count * extension_count) {
        if result.status == 200 {
            let s = format!(r#"{}.{}              (Status: {}) [Size: {}]"#, result.path, result.extension, result.status, result.size);
            println!("{}", s);
        }
    }

    println!();
    println!("Done. [{} milliseconds]", now.elapsed().as_millis());
}

fn get_line_count(file_path: &str) -> usize {
    let input = File::open(file_path).expect("Cannot open wordlist");
    let buffered = BufReader::new(input);
    let line_count = buffered.lines().count();
    line_count
}
