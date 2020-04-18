#!/usr/bin/env bash

set -e

cargo build

time ./target/debug/fwar
