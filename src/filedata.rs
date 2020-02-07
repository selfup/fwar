use std::time::Duration;

#[derive(Debug)]
pub struct FileData {
    pub modified: Duration,
    pub path: String,
}

impl FileData {
    pub fn new(modified: Duration, path: String) -> FileData {
        FileData {
            path: path,
            modified: modified,
        }
    }
}
