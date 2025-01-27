use std::{
    fs::File,
    path::{Path, PathBuf},
};

use ropey::Rope;
use serde::{Deserialize, Serialize};

/// Stores a point of interest in the project
#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub filepath: PathBuf,
    pub line_num: usize,
    pub context_before: Vec<String>,
    pub line: String,
    pub context_after: Vec<String>,
    pub note: String,
    pub id: u32,
}

pub fn read_bookmark_file(path: &Path) -> std::io::Result<Vec<Bookmark>> {
    let f = File::open(path)?;
    let bookmarks: Vec<Bookmark> = serde_json::from_reader(f)?;
    Ok(bookmarks)
}

pub fn write_bookmark_file(path: &Path, bookmarks: &Vec<Bookmark>) -> Result<(), std::io::Error> {
    let f = File::create(path)?;
    serde_json::to_writer_pretty(f, bookmarks)?;
    Ok(())
}

pub fn locate_bookmark(r: &Rope, bookmark: &Bookmark) -> Option<usize> {
    let line_num = (bookmark.line_num - 1).min(r.len_lines() - 1);

    // Most likely case, the line has not moved
    if r.line(line_num).to_string() == bookmark.line {
        return Some(line_num);
    }

    // Otherwise, iterate forwards and backwards in lockstep until a matching line is found
    let mut forward = r.lines();
    let mut backward = r.lines();
    forward.nth(line_num);
    backward.nth(line_num);
    let mut offset = 1;
    loop {
        let mut done = true;
        if let Some(line) = forward.next() {
            done = false;
            if line.to_string() == bookmark.line {
                return Some(line_num + offset);
            }
        }

        if let Some(line) = backward.prev() {
            done = false;
            if line.to_string() == bookmark.line {
                return Some(line_num - offset + 1); // Why is this 1 needed?
            }
        }

        if done {
            break;
        }
        offset += 1;
    }
    None
}
