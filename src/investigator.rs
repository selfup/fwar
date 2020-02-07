// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Regis Boudinot

use std::fs;
use std::io::{Error, ErrorKind, Result};
use std::path::Path;
use std::time::{Duration, SystemTime};

pub fn investigate(path: &Path) -> Result<Duration> {
    let metadata = fs::metadata(path)?;

    match metadata.modified() {
        Ok(modifed_time) => match modifed_time.duration_since(SystemTime::UNIX_EPOCH) {
            Ok(duration) => Ok(duration),
            Err(duration_err) => {
                let formatted_duration_err = format!("{:?}", duration_err);

                Err(Error::new(ErrorKind::Other, formatted_duration_err))
            }
        },
        Err(modified_time_err) => Err(Error::new(ErrorKind::Other, modified_time_err)),
    }
}
