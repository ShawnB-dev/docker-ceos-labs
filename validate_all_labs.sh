#!/bin/bash
# Script to validate all docker-compose configurations across labs

set -e

echo "🔍 Validating all lab configurations..."

for lab_dir in [0-1][0-9]-*; do
    if [ -d "$lab_dir" ]; then
        echo "--- Validating $lab_dir ---"
        (cd "$lab_dir" && docker compose config > /dev/null)
    fi
done

echo "✅ All labs validated successfully!"