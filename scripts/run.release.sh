#!/usr/bin/env bash

set -e

cargo build --release

time ./target/release/fwar
