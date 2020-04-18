// SPDX-License-Identifier: MIT
//
// Copyright (c) 2020 Regis Boudinot
use blake3;

pub fn checksum(file: &str) -> [u8; 32] {
  let hash1 = blake3::hash(file.as_bytes());

  let bytes = hash1.as_bytes().clone();

  bytes
}
