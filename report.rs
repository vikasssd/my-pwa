use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use crate::analyzer::has_syntax_errors;
use crate::config::{
    MAX_FILES, MAX_RUST_ANALYSIS_BYTES, MAX_TOTAL_CONTENT_BYTES, OUTPUT_FILE,
};

pub fn write_file_list(
    root: &Path,
    files: impl IntoIterator<Item = PathBuf>,
) -> io::Result<()> {
    let output = File::create(root.join(OUTPUT_FILE))?;
    let mut writer = BufWriter::new(output);
    let mut remaining_bytes = MAX_TOTAL_CONTENT_BYTES as u64;

    for (index, file_path) in files.into_iter().enumerate() {
        if index >= MAX_FILES {
            writeln!(writer, "\n[फ़ाइलों की सीमा पूरी हुई: {MAX_FILES}]")?;
            break;
        }

        let relative_path = file_path.strip_prefix(root).unwrap_or(&file_path);
        let is_rust_file =
            relative_path.extension().and_then(|ext| ext.to_str()) == Some("rs");

        let metadata = match fs::metadata(&file_path) {
            Ok(metadata) => metadata,
            Err(error) => {
                writeln!(
                    writer,
                    "\n===== {} | पढ़ने में त्रुटि: {error} =====",
                    relative_path.display()
                )?;
                continue;
            }
        };

        let mut file = match File::open(&file_path) {
            Ok(file) => file,
            Err(error) => {
                writeln!(
                    writer,
                    "\n===== {} | खोलने में त्रुटि: {error} =====",
                    relative_path.display()
                )?;
                continue;
            }
        };

        let mut sample = [0_u8; 8192];
        let sample_len = file.read(&mut sample)?;

        if sample[..sample_len].contains(&0) {
            writeln!(
                writer,
                "\n===== {} | बाइनरी फ़ाइल; छोड़ी गई =====",
                relative_path.display()
            )?;
            continue;
        }

        if remaining_bytes == 0 {
            writeln!(writer, "\n[रिपोर्ट की content सीमा पूरी हुई]")?;
            break;
        }

        file.seek(SeekFrom::Start(0))?;
        writeln!(writer, "\n===== {} =====", relative_path.display())?;

        let mut reader = BufReader::new(file).take(remaining_bytes);
        let mut line_bytes = Vec::new();
        let mut rust_source = String::new();
        let analyze_rust =
            is_rust_file && metadata.len() <= MAX_RUST_ANALYSIS_BYTES as u64;
        let mut file_bytes_read = 0_u64;
        let mut line_count = 0_u64;
        let mut word_count = 0_usize;

        loop {
            line_bytes.clear();

            let bytes_read = reader.read_until(b'\n', &mut line_bytes)?;
            if bytes_read == 0 {
                break;
            }

            file_bytes_read += bytes_read as u64;
            remaining_bytes = remaining_bytes.saturating_sub(bytes_read as u64);
            line_count += 1;

            let line = String::from_utf8_lossy(&line_bytes);
            word_count += line.split_whitespace().count();

            if analyze_rust {
                rust_source.push_str(&line);
            }

            writer.write_all(line.as_bytes())?;

            if remaining_bytes == 0 {
                break;
            }
        }

        if is_rust_file {
            let status = if !analyze_rust {
                "फ़ाइल 1 MiB से बड़ी; विश्लेषण नहीं किया"
            } else if file_bytes_read < metadata.len() {
                "फ़ाइल अधूरी पढ़ी गई; विश्लेषण नहीं किया"
            } else {
                match has_syntax_errors(&rust_source) {
                    Ok(true) => "syntax error मिला",
                    Ok(false) => "syntax सही",
                    Err(_) => "विश्लेषण नहीं हो पाया",
                }
            };

            writeln!(writer, "\n[Rust analysis: {status}]")?;
        }

        writeln!(
            writer,
            "[समाप्त: {line_count} पंक्तियाँ, {word_count} शब्द]"
        )?;

        if remaining_bytes == 0 {
            writeln!(writer, "\n[रिपोर्ट की content सीमा पूरी हुई]")?;
            break;
        }
    }

    Ok(())
}
