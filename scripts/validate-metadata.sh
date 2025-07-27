#!/bin/bash
# Validates all metadata.json files in exercises

set -e

echo "📋 Validating exercise metadata..."

FAILED=0

# Required fields in metadata.json
REQUIRED_FIELDS=(
    "id"
    "title"
    "description"
    "chapter"
    "exercise_number"
    "difficulty"
    "estimated_time_minutes"
    "concepts"
    "prerequisites"
    "exercise_type"
    "rust_book_refs"
    "hints"
    "testing"
    "validation"
)

# Valid exercise types
VALID_EXERCISE_TYPES=(
    "code_completion"
    "bug_fixing"
    "from_scratch"
    "code_review"
    "performance"
)

# Valid difficulty levels
VALID_DIFFICULTIES=(
    "beginner"
    "intermediate"
    "advanced"
)

# Function to validate metadata
validate_metadata() {
    local metadata_file="$1"
    local exercise_path=$(dirname "$metadata_file")
    local exercise_name=$(basename "$exercise_path")
    local chapter_name=$(basename "$(dirname "$exercise_path")")
    
    echo -n "  Checking $chapter_name/$exercise_name... "
    
    # Check if file is valid JSON
    if ! jq empty "$metadata_file" 2>/dev/null; then
        echo "❌ Invalid JSON"
        FAILED=1
        return 1
    fi
    
    # Check required fields
    for field in "${REQUIRED_FIELDS[@]}"; do
        if ! jq -e ".$field" "$metadata_file" >/dev/null 2>&1; then
            echo "❌ Missing required field: $field"
            FAILED=1
            return 1
        fi
    done
    
    # Validate exercise_type
    local exercise_type=$(jq -r '.exercise_type' "$metadata_file")
    if [[ ! " ${VALID_EXERCISE_TYPES[@]} " =~ " ${exercise_type} " ]]; then
        echo "❌ Invalid exercise_type: $exercise_type"
        FAILED=1
        return 1
    fi
    
    # Validate difficulty
    local difficulty=$(jq -r '.difficulty' "$metadata_file")
    if [[ ! " ${VALID_DIFFICULTIES[@]} " =~ " ${difficulty} " ]]; then
        echo "❌ Invalid difficulty: $difficulty"
        FAILED=1
        return 1
    fi
    
    # Validate ID format (should match directory structure)
    local expected_id="ch$(printf "%02d" $(jq -r '.chapter' "$metadata_file"))-ex$(printf "%02d" $(jq -r '.exercise_number' "$metadata_file"))-"
    local actual_id=$(jq -r '.id' "$metadata_file")
    if [[ ! "$actual_id" =~ ^$expected_id ]]; then
        echo "❌ ID doesn't match pattern: expected to start with $expected_id"
        FAILED=1
        return 1
    fi
    
    # Validate rust_book_refs structure
    if ! jq -e '.rust_book_refs.primary_chapter' "$metadata_file" >/dev/null 2>&1; then
        echo "❌ Missing rust_book_refs.primary_chapter"
        FAILED=1
        return 1
    fi
    
    # Check if URLs in rust_book_refs are valid (allow multiple Rust documentation sites)
    local urls=$(jq -r '.rust_book_refs.specific_sections[]?.url // empty' "$metadata_file")
    for url in $urls; do
        if [[ ! "$url" =~ ^https://doc\.rust-lang\.org/(book|edition-guide|std|reference|cargo)/ ]]; then
            echo "❌ Invalid Rust Book URL: $url"
            FAILED=1
            return 1
        fi
    done
    
    echo "✓"
}

# Find all metadata.json files
for metadata_file in exercises/ch*/ex*/metadata.json; do
    if [[ -f "$metadata_file" ]]; then
        validate_metadata "$metadata_file"
    fi
done

echo ""
if [[ $FAILED -eq 0 ]]; then
    echo "✅ All metadata files are valid!"
    exit 0
else
    echo "❌ Some metadata files have validation errors"
    exit 1
fi