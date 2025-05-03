#!/usr/bin/env bash
# Recursively print all files in ./src with fenced code blocks

find ./src -type f -name '*.rs' | sort | while read -r file; do
  echo "$file"
  echo '```rust'
  cat "$file"
  echo '```'
  echo
done
