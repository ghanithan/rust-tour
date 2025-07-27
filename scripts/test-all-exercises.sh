#!/bin/bash
# Tests all exercises based on their type
# For bug_fixing exercises, we expect compilation to fail
# For other types, we run cargo test

set -e

echo "🧪 Testing all exercises..."

FAILED=0
TOTAL=0
PASSED=0

# Function to test individual exercise
test_exercise() {
    local exercise_path="$1"
    local exercise_name=$(basename "$exercise_path")
    local chapter_name=$(basename "$(dirname "$exercise_path")")
    
    TOTAL=$((TOTAL + 1))
    echo -n "  Testing $chapter_name/$exercise_name... "
    
    # Get exercise type from metadata
    if [[ ! -f "$exercise_path/metadata.json" ]]; then
        echo "❌ Missing metadata.json"
        FAILED=$((FAILED + 1))
        return 1
    fi
    
    local exercise_type=$(jq -r '.exercise_type' "$exercise_path/metadata.json" 2>/dev/null)
    
    # Change to exercise directory
    cd "$exercise_path"
    
    case "$exercise_type" in
        "bug_fixing")
            # For bug_fixing exercises, we expect compilation to fail
            if cargo check --quiet 2>/dev/null; then
                echo "❌ Bug fixing exercise compiles (should have errors)"
                FAILED=$((FAILED + 1))
                return 1
            else
                echo "✓ (compilation errors expected)"
                PASSED=$((PASSED + 1))
            fi
            ;;
            
        "code_completion"|"from_scratch")
            # These might have unimplemented!() or TODOs, so cargo check is enough
            if cargo check --quiet 2>/dev/null; then
                echo "✓"
                PASSED=$((PASSED + 1))
            else
                # Check if it's a syntax error or just unimplemented
                if cargo check 2>&1 | grep -q "unimplemented\|todo\|TODO"; then
                    echo "✓ (contains TODOs)"
                    PASSED=$((PASSED + 1))
                else
                    echo "❌ Compilation error"
                    FAILED=$((FAILED + 1))
                    return 1
                fi
            fi
            ;;
            
        "code_review"|"performance")
            # These should compile and pass tests
            if cargo test --quiet 2>/dev/null; then
                echo "✓"
                PASSED=$((PASSED + 1))
            else
                echo "❌ Tests failed"
                FAILED=$((FAILED + 1))
                return 1
            fi
            ;;
            
        *)
            echo "❌ Unknown exercise type: $exercise_type"
            FAILED=$((FAILED + 1))
            return 1
            ;;
    esac
    
    # Return to original directory
    cd - >/dev/null
}

# Store current directory
ORIGINAL_DIR=$(pwd)

# Find all exercise directories
for chapter_dir in exercises/ch*/; do
    if [[ -d "$chapter_dir" ]]; then
        for exercise_dir in "$chapter_dir"ex*/; do
            if [[ -d "$exercise_dir" ]]; then
                test_exercise "$ORIGINAL_DIR/$exercise_dir"
            fi
        done
    fi
done

# Return to original directory
cd "$ORIGINAL_DIR"

echo ""
echo "📊 Test Summary:"
echo "   Total exercises: $TOTAL"
echo "   Passed: $PASSED"
echo "   Failed: $FAILED"
echo ""

if [[ $FAILED -eq 0 ]]; then
    echo "✅ All exercises tested successfully!"
    exit 0
else
    echo "❌ Some exercises failed testing"
    exit 1
fi