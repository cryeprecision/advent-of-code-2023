#!/bin/bash
set -e

cargo build --release
echo

cargo build
echo

files_release=($(find ./target/release -name "day-*-part-*" ! -name "*.d" ! -name "*.exe"))
files_debug=($(find ./target/debug -name "day-*-part-*" ! -name "*.d" ! -name "*.exe"))

echo "-> Release <-"
for file in "${files_release[@]}"; do
    # Do two dry-runs
    for ((i=0; i<2; i++)); do
        $file > /dev/null
    done
    # Output the third run
    $file
done
echo

echo "-> Debug <-"
for file in "${files_debug[@]}"; do
    # Do two dry-runs
    for ((i=0; i<2; i++)); do
        $file > /dev/null
    done
    # Output the third run
    $file
done
