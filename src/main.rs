// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Regis Boudinot

mod filedata;
mod investigator;

use filedata::FileData;

fn main() -> std::io::Result<()> {
    let mut files = vec![];

    for entry in std::fs::read_dir("artifact")? {
        match entry {
            Ok(file) => {
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

    println!("{:?}", files);

    Ok(())
}
