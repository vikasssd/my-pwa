// Settings, report aur scanner ke alag modules.
mod config;
mod report;
mod scanner;
mod analyzer;


use std::{env, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let root = env::current_dir()?;
    let files = scanner::collect_files(&root);

    report::write_file_list(&root, files)?;

    println!("folder-index.txt ban gayi.");
    Ok(())
}
