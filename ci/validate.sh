#!/bin/bash

set -eu

for file in FRENCH/src/*.md ; do
    echo Checking references in "$file"
    cargo run --quiet --bin link2print < "$file" > /dev/null
done