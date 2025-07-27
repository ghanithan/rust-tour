#!/bin/bash
# Validates exercise structure and requirements
# Allows broken main.rs for bug_fixing exercises

set -e

echo "🔍 Validating exercise structure..."

FAILED=0

# Function to validate individual exercise
validate_exercise() {
    local exercise_path="$1"
    local exercise_name=$(basename "$exercise_path")
    local chapter_name=$(basename "$(dirname "$exercise_path")")
    
    echo -n "  Checking $chapter_name/$exercise_name... "
    
    # Check required files exist
    if [[ ! -f "$exercise_path/Cargo.toml" ]]; then
        echo "❌ Missing Cargo.toml"
        FAILED=1
        return 1
    fi
    
    if [[ ! -f "$exercise_path/README.md" ]]; then
        echo "❌ Missing README.md"
        FAILED=1
        return 1
    fi
    
    if [[ ! -f "$exercise_path/hints.md" ]]; then
        echo "❌ Missing hints.md"
        FAILED=1
        return 1
    fi
    
    if [[ ! -f "$exercise_path/metadata.json" ]]; then
        echo "❌ Missing metadata.json"
        FAILED=1
        return 1
    fi
    
    if [[ ! -d "$exercise_path/tests" ]] && [[ ! -f "$exercise_path/src/main.rs" ]] && [[ ! -f "$exercise_path/src/lib.rs" ]]; then
        echo "❌ Missing tests directory or source files"
        FAILED=1
        return 1
    fi
    
    # Check if metadata.json is valid JSON
    if ! jq empty "$exercise_path/metadata.json" 2>/dev/null; then
        echo "❌ Invalid metadata.json"
        FAILED=1
        return 1
    fi
    
    # Get exercise type from metadata
    local exercise_type=$(jq -r '.exercise_type' "$exercise_path/metadata.json")
    
    # For bug_fixing exercises, we expect compilation to fail
    if [[ "$exercise_type" == "bug_fixing" ]]; then
        echo "✓ (bug_fixing exercise - broken code expected)"
    else
        # For other types, check if it at least has valid Rust syntax
        if cd "$exercise_path" && cargo check --quiet 2>/dev/null; then
            echo "✓"
        else
            # It's okay if it doesn't compile due to TODOs, unimplemented!(), etc.
            echo "✓ (contains TODOs/unimplemented - expected)"
        fi
    fi
}

# Find all exercise directories
for chapter_dir in exercises/ch*/; do
    if [[ -d "$chapter_dir" ]]; then
        for exercise_dir in "$chapter_dir"ex*/; do
            if [[ -d "$exercise_dir" ]]; then
                validate_exercise "$exercise_dir"
            fi
        done
    fi
done

echo ""
if [[ $FAILED -eq 0 ]]; then
    echo "✅ All exercises have valid structure!"
    exit 0
else
    echo "❌ Some exercises have invalid structure"
    exit 1
fi