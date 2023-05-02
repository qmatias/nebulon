#!/bin/bash

set -euo pipefail

echo "Copying assets"
if [ ! -d ../../assets ]; then
    echo "Error: work dir must be native launcher directory before running this script"
    exit 1
fi
cp -r ../../assets/ assets/

cargo run
