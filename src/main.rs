// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Regis Boudinot

mod investigator;

use rayon::prelude::*;
use std::io::Result;
use std::time::Duration;
use walkdir::WalkDir;

fn main() {
    let mut files = vec![];
    let mut timestamps: Vec<Result<Duration>> = vec![];

    for entry in WalkDir::new("artifact").into_iter().filter_map(|e| e.ok()) {
        files.push(entry);
    }

    files
        .par_iter()
        .map(|file| investigator::investigate(file.path()))
        .collect_into_vec(&mut timestamps);

    println!("{:?}", timestamps);
}
