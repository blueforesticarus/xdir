use std::env::args;

use walkdir::{DirEntry, WalkDir};

fn main() {
    let mut a = args();
    let a = a.nth(1).unwrap_or(".".to_string());

    let mut stack: Vec<DirEntry> = Vec::new();
    // Each item yielded by the iterator is either a directory entry or an
    // error, so either print the path or the error.
    for entry in WalkDir::new(a).into_iter().filter_map(|e| e.ok()) {
        loop {
            let Some(parent) = stack.last() else { break };

            if Some(parent.path()) == entry.path().parent() {
                break;
            }

            stack.pop();
            println!("</dir>")
        }

        if !entry.file_type().is_dir() {
            println!(
                "<file name=\"{}\"></file>",
                entry
                    .path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            );
        } else {
            println!(
                "<dir name=\"{}\">",
                entry
                    .path()
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            );
        };

        if entry.file_type().is_dir() {
            stack.push(entry);
        }
    }
    for _ in stack {
        println!("</dir>")
    }
}
