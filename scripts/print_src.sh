#!/usr/bin/env bash
# Recursively print all files in ./src with fenced code blocks

shopt -s globstar nullglob

for file in ./src/**/*; do
  if [[ -f "$file" ]]; then
    printf '%s\n' "$file"
    printf '```rust\n'
    cat "$file"
    printf '\n```\n\n'
  fi
done
