// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Regis Boudinot

mod investigator;

fn main() {
    match investigator::investigate() {
        Ok(result) => println!("{:?}", result),
        Err(err) => println!("{:?}", err),
    }
}
