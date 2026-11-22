pub const OUTPUT_FILE: &str = "folder-index.txt";

pub const IGNORED_DIRS: &[&str] = &[
    ".git",
    "target",
    "node_modules",
    "vendor",
    "build",
    "dist",
    ".cache",
];

pub const MAX_FILES: usize = 50_000;
pub const MAX_RUST_ANALYSIS_BYTES: u64 = 1_048_576;
pub const MAX_TOTAL_CONTENT_BYTES: usize = 64 * 1024 * 1024; // कुल 64 MiB
