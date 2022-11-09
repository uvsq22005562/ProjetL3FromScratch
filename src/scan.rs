use walkdir::{DirEntry, WalkDir};
use std::path::Path;

pub fn scan(path: &Path) -> Vec<Result<walkdir::DirEntry, walkdir::Error>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        files.push(entry);
    }
    files
}
