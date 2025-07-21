#!/bin/bash
set -e

echo "🧪 Testing Rust Learning Platform Exercises"
echo "==========================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Test a single exercise
test_exercise() {
    local exercise_path="$1"
    local exercise_name=$(basename "$exercise_path")
    
    print_status "Testing exercise: $exercise_name"
    
    cd "$exercise_path"
    
    # Test that metadata is valid JSON
    if [ -f "metadata.json" ]; then
        if ! python3 -m json.tool metadata.json > /dev/null 2>&1; then
            print_error "Invalid JSON in metadata.json for $exercise_name"
            cd - > /dev/null
            return 1
        fi
    else
        print_error "Missing metadata.json for $exercise_name"
        cd - > /dev/null
        return 1
    fi
    
    # Test that Cargo.toml is valid
    if [ -f "Cargo.toml" ]; then
        if ! cargo check --quiet > /dev/null 2>&1; then
            print_warning "Exercise $exercise_name may have compilation issues (expected for bug-fixing exercises)"
        fi
    else
        print_error "Missing Cargo.toml for $exercise_name"
        cd - > /dev/null
        return 1
    fi
    
    # Check for required files
    required_files=("README.md" "src/main.rs")
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            print_error "Missing required file: $file in $exercise_name"
            cd - > /dev/null
            return 1
        fi
    done
    
    # Check for solutions
    if [ ! -d "solutions" ] || [ ! -f "solutions/reference.rs" ]; then
        print_warning "Missing solutions directory or reference solution for $exercise_name"
    fi
    
    # Check for hints
    if [ ! -f "src/hints.md" ]; then
        print_warning "Missing hints file for $exercise_name"
    fi
    
    # Test solutions if they exist
    if [ -f "solutions/reference.rs" ]; then
        print_status "Testing reference solution for $exercise_name"
        
        # Backup original main.rs
        cp src/main.rs src/main.rs.backup
        
        # Copy reference solution
        cp solutions/reference.rs src/main.rs
        
        # Test that solution compiles and runs
        if cargo check --quiet > /dev/null 2>&1; then
            if cargo run --quiet > /dev/null 2>&1; then
                print_success "Reference solution works for $exercise_name"
            else
                print_error "Reference solution runs but may have runtime issues for $exercise_name"
            fi
        else
            print_error "Reference solution doesn't compile for $exercise_name"
        fi
        
        # Restore original main.rs
        mv src/main.rs.backup src/main.rs
    fi
    
    print_success "Exercise $exercise_name validation complete"
    cd - > /dev/null
    return 0
}

# Test exercise framework
test_framework() {
    print_status "Testing exercise framework"
    
    if [ -d "exercise-framework" ]; then
        cd exercise-framework
        if cargo test --quiet; then
            print_success "Exercise framework tests pass"
        else
            print_error "Exercise framework tests failed"
            cd - > /dev/null
            return 1
        fi
        cd - > /dev/null
    else
        print_warning "Exercise framework directory not found"
    fi
}

# Main testing logic
main() {
    # Test framework first
    test_framework
    
    # Find all exercises
    exercise_count=0
    success_count=0
    
    print_status "Searching for exercises..."
    
    # Test Chapter 1 exercises
    for exercise in exercises/ch01_getting_started/ex*; do
        if [ -d "$exercise" ]; then
            exercise_count=$((exercise_count + 1))
            if test_exercise "$exercise"; then
                success_count=$((success_count + 1))
            fi
        fi
    done
    
    # Test Chapter 3 exercises
    for exercise in exercises/ch03_common_concepts/ex*; do
        if [ -d "$exercise" ]; then
            exercise_count=$((exercise_count + 1))
            if test_exercise "$exercise"; then
                success_count=$((success_count + 1))
            fi
        fi
    done
    
    # Summary
    echo ""
    print_status "Testing Summary:"
    print_status "Total exercises: $exercise_count"
    print_status "Successful: $success_count"
    print_status "Failed: $((exercise_count - success_count))"
    
    if [ "$success_count" -eq "$exercise_count" ]; then
        print_success "All exercises passed validation! 🎉"
        return 0
    else
        print_error "Some exercises failed validation"
        return 1
    fi
}

# Run main function
main "$@"