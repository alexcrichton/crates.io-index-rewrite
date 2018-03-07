use std::fs;
use std::io::{self, Write};
use std::path::Path;
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
    remove_dir_all("snapshot/tmp".as_ref());

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
    remove_dir_all("squashed/tmp".as_ref());
}

fn print(dur: Duration) {
    println!(" {}.{:03}s", dur.as_secs(), dur.subsec_nanos() / 1_000_000);
}

fn remove_dir_all(p: &Path) {
    for e in p.read_dir().unwrap() {
        let e = e.unwrap();
        let path = e.path();
        if e.file_type().unwrap().is_dir() {
            remove_dir_all(&path);
        } else {
            remove_file(&path);
        }
    }
    fs::remove_dir(p).unwrap();

    fn remove_file(p: &Path) {
        let mut err = match fs::remove_file(p) {
            Ok(()) => return,
            Err(e) => e,
        };

        if err.kind() != io::ErrorKind::PermissionDenied {
            panic!("failed to remove {}: {}", p.display(), err)
        }
        if set_not_readonly(p).unwrap_or(false) {
            match fs::remove_file(p) {
                Ok(()) => return,
                Err(e) => err = e,
            }
        }

        panic!("failed to remove file {}: {}", p.display(), err);
    }

    fn set_not_readonly(p: &Path) -> io::Result<bool> {
        let mut perms = p.metadata()?.permissions();
        if !perms.readonly() {
            return Ok(false)
        }
        perms.set_readonly(false);
        fs::set_permissions(p, perms)?;
        Ok(true)
    }
}
