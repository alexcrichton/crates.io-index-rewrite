extern crate remove_dir_all;

use std::io::{self, Write};
use std::process::Command;
use std::time::{Duration, Instant};

fn main() {
    print!("A completely fresh checkout of the index takes...");
    io::stdout().flush().unwrap();
    let start = Instant::now();
    let status = Command::new("cargo")
        .current_dir("snapshot")
        .arg("update")
        .env("CARGO_HOME", "tmp")
        .output()
        .unwrap();
    assert!(status.status.success());
    print(start.elapsed());
    remove_dir_all::remove_dir_all("snapshot/tmp").unwrap();

    print!("Cloning an index with a history of length 1 takes...");
    io::stdout().flush().unwrap();
    let start = Instant::now();
    let status = Command::new("cargo")
        .current_dir("squashed")
        .arg("update")
        .env("CARGO_HOME", "tmp")
        .output()
        .unwrap();
    assert!(status.status.success());
    print(start.elapsed());
    remove_dir_all::remove_dir_all("squashed/tmp").unwrap();
}

fn print(dur: Duration) {
    println!(" {}.{:03}s", dur.as_secs(), dur.subsec_nanos() / 1_000_000);
}
