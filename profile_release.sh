#!/bin/bash
set -e

rm callgrind.out.*
cargo build --profile release-with-debug --bin $1

valgrind --tool=callgrind ./target/release-with-debug/$1
callgrind_annotate --inclusive=yes --auto=yes "$(find ./ -maxdepth 1 -name "callgrind.out.[0-9]*")"
