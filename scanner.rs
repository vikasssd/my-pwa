use std::path::{Path, PathBuf};
use walkdir::WalkDir;

use crate::config::{IGNORED_DIRS, OUTPUT_FILE};

pub fn collect_files(root: &Path) -> impl Iterator<Item = PathBuf> + '_ {
    WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| {
            !entry
                .file_name()
                .to_str()
                .is_some_and(|name| IGNORED_DIRS.contains(&name))
        })
        .filter_map(|result| match result {
            Ok(entry) => Some(entry),
            Err(error) => {
                eprintln!("Scan warning: {error}");
                None
            }
        })
        .filter(|entry| entry.file_type().is_file())
        .filter(|entry| {
            entry.path().extension().and_then(|ext| ext.to_str()) != Some("swp")
        })
        .filter(|entry| entry.file_name().to_str() != Some(OUTPUT_FILE))
        .map(|entry| entry.into_path())
}
