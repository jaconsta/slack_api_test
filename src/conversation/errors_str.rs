use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct QueryError {
    details: String,
}

impl QueryError {
    pub fn new(msg: &str) -> QueryError {
        QueryError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for QueryError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct SlackChannelError {
    details: String,
}

impl SlackChannelError {
    pub fn new(msg: &str) -> SlackChannelError {
        SlackChannelError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for SlackChannelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SlackChannelError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(Debug)]
pub struct FileSystemError {
    details: String,
}

impl FileSystemError {
    pub fn new(msg: &str) -> FileSystemError {
        FileSystemError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for FileSystemError {
    fn description(&self) -> &str {
        &self.details
    }
}
