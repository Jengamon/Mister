#!/bin/bash
set -ev
cargo build --verbose
cargo test -p mister-core --verbose
cargo test -p mister-gui --verbose
