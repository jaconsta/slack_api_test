use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::conversation::{entity::users::User, errors_str::FileSystemError};

static FILE_PATH: &'static str = "static/storage";
static FILE_NAME: &'static str = "static/storage/users_cache.txt";

pub fn read_cache() -> Result<Vec<User>, FileSystemError> {
    let file = File::open(FILE_NAME);
    if let Err(error) = file {
        println!("{:?}", error);
        return Err(FileSystemError::new("Error opening file. {:?}"));
    }
    let file = file.unwrap();

    let mut slack_users = Vec::new();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Err(e) = line {
            println!("error load users from csv {:?}", e);
            continue;
        }
        let l = line.unwrap();

        let user_line: Vec<&str> = l.split(",").collect();
        if user_line.is_empty() || !user_line[0].starts_with("C") {
            continue;
        }
        let user = User::new(user_line[0], user_line[1], user_line[2] == "true");
        slack_users.push(user);
    }

    return Ok(slack_users);
}
