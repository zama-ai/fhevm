#!/bin/bash

# Check for required tools
required_tools=("npm" "jq" "docker" "python3" "cargo" "npx")

for tool in "${required_tools[@]}"; do
  if ! command -v "$tool" &> /dev/null; then
    echo >&2 "Error: $tool is not installed."
    exit 1
  fi
done

