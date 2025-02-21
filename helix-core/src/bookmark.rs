use std::{
    fs::File,
    path::PathBuf,
};

use ropey::Rope;
use serde::{Deserialize, Serialize};

/// Stores a line of interest in the workspace
#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    /// Filepath relative to the workspace root
    pub filepath: PathBuf,
    /// 1-indexed line number
    pub line_num: usize,
    /// Bookmarked text, with leading and trailing whitespace trimmed
    pub line: String,
    /// Note to associate with line, used to search bookmarks
    pub note: String,
    /// Unique identifier
    pub id: u32,
}

pub fn bookmark_file() -> PathBuf {
    helix_loader::find_workspace().0.join(".helix").join("bookmarks.json")
}

pub fn read_bookmark_file() -> std::io::Result<Vec<Bookmark>> {
    let f = File::open(bookmark_file())?;
    let bookmarks: Vec<Bookmark> = serde_json::from_reader(f)?;
    Ok(bookmarks)
}

pub fn write_bookmark_file(bookmarks: &Vec<Bookmark>) -> Result<(), std::io::Error> {
    let f = File::create(bookmark_file())?;
    serde_json::to_writer_pretty(f, bookmarks)?;
    Ok(())
}

pub fn locate_bookmark(r: &Rope, bookmark: &Bookmark) -> Option<usize> {
    let line_num = (bookmark.line_num - 1).min(r.len_lines() - 1);

    // Most likely case, the line has not moved
    if r.line(line_num).to_string().trim() == bookmark.line {
        return Some(line_num);
    }

    // Otherwise, iterate forwards and backwards in lockstep to find the
    // closest matching line
    let mut forward = r.lines();
    let mut backward = r.lines();
    forward.nth(line_num);
    backward.nth(line_num);
    let mut offset = 1;
    loop {
        let mut done = true;
        if let Some(line) = forward.next() {
            done = false;
            if line.to_string().trim() == bookmark.line {
                return Some(line_num + offset);
            }
        }

        if let Some(line) = backward.prev() {
            done = false;
            if line.to_string().trim() == bookmark.line {
                return Some(line_num - offset + 1);
            }
        }

        if done {
            return None;
        }
        offset += 1;
    }
}
