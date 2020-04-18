// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Regis Boudinot

mod filedata;
mod investigator;

use filedata::FileData;

use std::io::{Error, ErrorKind};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut files: &mut Vec<filedata::FileData> = &mut vec![];

    match investigate_dir(&Path::new("artifact"), &mut files) {
        Ok(_result) => (),
        Err(err) => panic!(err),
    };

    println!("{}", files.len());

    Ok(())
}

fn investigate_dir<'a>(dir: &Path, files: &mut Vec<filedata::FileData>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        match entry {
            Ok(file) => {
                if file.path().is_dir() {
                    let dir_path = file.path();
                    match investigate_dir(&dir_path, files) {
                        Ok(_result) => (),
                        Err(err) => {
                            let error = format!("{:?}", err);
                            println!("{}", error);
                            Error::new(ErrorKind::Other, error);
                        }
                    };
                }

                let modified = match investigator::investigate(&file.path()) {
                    Ok(result) => result,
                    Err(err) => panic!(err),
                };
                let file_data = match file.path().to_str() {
                    Some(str) => FileData::new(modified, String::from(str)),
                    None => panic!("no path string"),
                };

                files.push(file_data);
            }
            Err(err) => panic!(err),
        }
    }

    Ok(())
}
