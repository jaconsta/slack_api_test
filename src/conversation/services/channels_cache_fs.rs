use crate::conversation::errors_str::FileSystemError;
use std::fs::{self, OpenOptions};
use std::io::{BufRead, BufReader};
use std::{fs::File, io::Write, path::Path};

static FILE_PATH: &'static str = "static/storage";
static FILE_NAME: &'static str = "static/storage/channels_cache.txt";

pub struct ChannelStorage {
    // Slack unique id
    pub channel_id: String,
    // Channel name
    pub name: String,
    // Was it manually inserted
    pub custom: bool,
    // Ignore this channel when fetching additional information
    pub ignore: bool,
}

fn echo(s: &str, path: &Path) -> Result<(), FileSystemError> {
    let mut file = match OpenOptions::new().write(true).create_new(true).open(path) {
        Ok(f) => f,
        _ => return Err(FileSystemError::new("Failed opening the object path.")),
    };

    // let mut f = match File::open(path) {
    //     Ok(f) => f,
    //     _ => return Err(FileSystemError::new("Failed opening the object path.")),
    // };

    if let Err(e) = file.write_all(s.as_bytes()) {
        println!("{:?}", e);
        return Err(FileSystemError::new("Failed to write file contents."));
    }

    return Ok(());
}

pub fn create_cache(storage: &Vec<ChannelStorage>) -> Result<(), FileSystemError> {
    let as_lines: Vec<String> = storage
        .iter()
        .map(|s| format!("{},{},{}", s.channel_id, s.name, s.custom))
        .collect();
    let line_items: Vec<&str> = as_lines.iter().map(|f| f.as_str()).collect();
    if let Err(e) = store_cache(line_items) {
        println!("{:?}", e);
        return Err(FileSystemError::new("Failed to create file cache."));
    }

    Ok(())
}
fn store_cache(lines: Vec<&str>) -> Result<(), FileSystemError> {
    // mkdir -p static/storage/
    if let Err(e) = fs::create_dir_all(FILE_PATH) {
        println!("{:?}", e);
        return Err(FileSystemError::new("Error creating storage folder"));
    }

    println!("echo [content] > static/storage/[filename.ext]");
    let path = &Path::new(FILE_NAME);
    let content = lines.join("\n");
    echo(&content, path)?;

    return Ok(());
}

pub fn read_cache() -> Result<Vec<ChannelStorage>, FileSystemError> {
    let file = File::open(FILE_NAME);
    if let Err(error) = file {
        println!("{:?}", error);
        return Err(FileSystemError::new("Error opening file. {:?}"));
    }
    let file = file.unwrap();

    let mut slack_channels = Vec::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Err(e) = line {
            println!("eeor {:?}", e);
            continue;
        }
        let l = line.unwrap();

        let channel_as_line: Vec<&str> = l.split(",").collect();
        if channel_as_line.is_empty() || !channel_as_line[0].starts_with("C") {
            continue;
        }
        let channel_storage = ChannelStorage {
            channel_id: channel_as_line[0].try_into().unwrap_or("".to_string()),
            name: channel_as_line[1].try_into().unwrap_or("".to_string()),
            custom: channel_as_line[2] == "true",
            ignore: channel_as_line[3] == "true",
        };
        slack_channels.push(channel_storage);
    }

    return Ok(slack_channels);
}
