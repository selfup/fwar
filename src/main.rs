// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Regis Boudinot
mod checksum;
mod filedata;
mod investigator;

use filedata::FileData;

use std::fs::File;
use std::io::Read;
use std::io::{Error, ErrorKind};
use std::path::Path;
use std::str;

fn main() -> std::io::Result<()> {
    let mut files: &mut Vec<filedata::FileData> = &mut vec![];

    match investigate_dir(&Path::new("artifact"), &mut files) {
        Ok(_result) => (),
        Err(err) => panic!(err),
    };

    println!("[");
    println!("\"ln: {}\",", files.len());

    for file in files {
        if file.is_dir != true {
            let f = File::open(file.path.clone())?;
            let f_bytes = f.bytes();
            let mut file_bytes: Vec<u8> = vec![];

            for byte in f_bytes {
                match byte {
                    Ok(b) => file_bytes.push(b),
                    Err(_err) => {
                        println!("[\"ib: {}\"],", file.path.clone());
                        ()
                    }
                }
            }

            if let Ok(valid_contents) = str::from_utf8(&file_bytes) {
                let result = checksum::checksum(valid_contents);
                println!("{:?},", result);
            } else {
                println!("\"ie: {}\",", file.path);
            }
        }
    }

    println!("\"fi\"");
    println!("]");
    Ok(())
}

fn investigate_dir<'a>(dir: &Path, files: &mut Vec<filedata::FileData>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        match entry {
            Ok(file) => {
                let is_dir = file.path().is_dir();

                if is_dir {
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
                    Some(str) => FileData::new(is_dir, modified, String::from(str)),
                    None => panic!("no path string"),
                };

                files.push(file_data);
            }
            Err(err) => panic!(err),
        }
    }

    Ok(())
}
