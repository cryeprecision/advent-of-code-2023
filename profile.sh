#!/bin/bash
set -e

rm callgrind.out.*
cargo build --bin $1

valgrind --tool=callgrind ./target/debug/$1
callgrind_annotate --inclusive=yes --auto=yes "$(find ./ -maxdepth 1 -name "callgrind.out.[0-9]*")"
