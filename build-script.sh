#!/bin/bash

echo "\n\n>>> Building a release version and stripping symbols..."
cargo build --release && strip ./target/release/first
