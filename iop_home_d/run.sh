#!/bin/bash

cargo build
RUST_LOG=debug RUST_BACKTRACE=1 target/debug/iop_home_d