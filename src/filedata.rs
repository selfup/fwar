// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Regis Boudinot

use std::time::Duration;

#[derive(Debug)]
pub struct FileData {
    pub is_dir: bool,
    pub modified: Duration,
    pub path: String,
}

impl FileData {
    pub fn new(is_dir: bool, modified: Duration, path: String) -> FileData {
        FileData {
            is_dir: is_dir,
            path: path,
            modified: modified,
        }
    }
}
