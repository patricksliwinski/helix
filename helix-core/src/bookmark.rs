use std::{fs::File, path::{PathBuf, Path}};

use serde::{Serialize, Deserialize};

/// Stores a point of interest in the project
#[derive(Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub filepath: PathBuf,
    pub line_num: usize,
    pub context_before: Vec<String>,
    pub line: String,
    pub context_after: Vec<String>,
    pub note: String
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
