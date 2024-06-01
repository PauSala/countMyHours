use anyhow::{bail, Ok, Result};
use colored::Colorize;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::{self, Seek, SeekFrom};
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::time_utils::Time;

static DATA_FILE_NAME: &str = ".cmh.data";
static CONFIG_FILE_NAME: &str = ".cmh.config.json";

fn get_data_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    home_dir.join(DATA_FILE_NAME)
}

pub fn get_config_file_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Home directory not found");
    home_dir.join(CONFIG_FILE_NAME)
}

pub fn read_counter() -> Result<Time> {
    let path = get_data_file_path();

    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let last_line = reader.lines().filter_map(Result::ok).last();
        match last_line {
            Some(line) => {
                let time = Time::from_str(&line)?;
                Ok(time)
            }
            None => bail!("File is empty"),
        }
    } else {
        let mut file = File::create(path)?;
        let content = "00:00";
        writeln!(file, "{}", content)?;
        let time = Time::from_str(&content)?;
        Ok(time)
    }
}

pub fn append_to_file(content: &str) -> Result<()> {
    let path = get_data_file_path();
    let mut file = OpenOptions::new().write(true).append(true).open(path)?;

    writeln!(file, "{}", content)?;
    Ok(())
}

pub fn delete_last_two_lines() -> Result<()> {
    let path = get_data_file_path();
    let mut file = OpenOptions::new().read(true).write(true).open(path)?;
    let reader = io::BufReader::new(&file);

    let lines: Result<Vec<String>, _> = reader.lines().collect();
    let mut lines = lines?;

    if lines.len() < 2 {
        println!(
            "{} {}",
            "warning:".white(),
            "Not enough data to undo".white()
        );
        return Ok(());
    }

    lines.truncate(lines.len() - 2);

    file.seek(SeekFrom::Start(0))?;
    file.set_len(0)?;

    for line in lines {
        writeln!(file, "{}", line)?;
    }

    Ok(())
}
