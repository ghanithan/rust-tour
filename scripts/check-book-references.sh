#!/bin/bash
# Validates Rust Book references in metadata.json files
# Checks that URLs are valid and chapters exist

set -e

echo "📚 Checking Rust Book references..."

FAILED=0
TOTAL=0
WARNINGS=0

# Valid Rust Book chapter patterns
# Based on https://doc.rust-lang.org/book/
VALID_CHAPTERS=(
    "title-page"
    "foreword"
    "ch00-00-introduction"
    "ch01-00-getting-started"
    "ch01-01-installation"
    "ch01-02-hello-world"
    "ch01-03-hello-cargo"
    "ch02-00-guessing-game-tutorial"
    "ch03-00-common-programming-concepts"
    "ch03-01-variables-and-mutability"
    "ch03-02-data-types"
    "ch03-03-how-functions-work"
    "ch03-04-comments"
    "ch03-05-control-flow"
    "ch04-00-understanding-ownership"
    "ch04-01-what-is-ownership"
    "ch04-02-references-and-borrowing"
    "ch04-03-slices"
    "ch05-00-structs"
    "ch05-01-defining-structs"
    "ch05-02-example-structs"
    "ch05-03-method-syntax"
    "ch06-00-enums"
    "ch06-01-defining-an-enum"
    "ch06-02-match"
    "ch06-03-if-let"
    "ch07-00-managing-growing-projects-with-packages-crates-and-modules"
    "ch07-01-packages-and-crates"
    "ch07-02-defining-modules-to-control-scope-and-privacy"
    "ch07-03-paths-for-referring-to-an-item-in-the-module-tree"
    "ch07-04-bringing-paths-into-scope-with-the-use-keyword"
    "ch07-05-separating-modules-into-different-files"
    "ch08-00-common-collections"
    "ch08-01-vectors"
    "ch08-02-strings"
    "ch08-03-hash-maps"
    "ch09-00-error-handling"
    "ch09-01-unrecoverable-errors-with-panic"
    "ch09-02-recoverable-errors-with-result"
    "ch09-03-to-panic-or-not-to-panic"
    "ch10-00-generics"
    "ch10-01-syntax"
    "ch10-02-traits"
    "ch10-03-lifetime-syntax"
    "ch11-00-testing"
    "ch11-01-writing-tests"
    "ch11-02-running-tests"
    "ch11-03-test-organization"
    "ch12-00-an-io-project"
    "ch12-01-accepting-command-line-arguments"
    "ch12-02-reading-a-file"
    "ch12-03-improving-error-handling-and-modularity"
    "ch12-04-developing-the-library-functionality-with-test-driven-development"
    "ch12-05-working-with-environment-variables"
    "ch12-06-writing-error-messages-to-standard-error-instead-of-standard-output"
    "ch13-00-functional-features"
    "ch13-01-closures"
    "ch13-02-iterators"
    "ch13-03-improving-our-io-project"
    "ch13-04-comparing-performance-loops-vs-iterators"
    "ch14-00-more-about-cargo"
    "ch14-01-release-profiles"
    "ch14-02-publishing-to-crates-io"
    "ch14-03-cargo-workspaces"
    "ch14-04-installing-binaries-from-crates-io"
    "ch14-05-extending-cargo"
    "ch15-00-smart-pointers"
    "ch15-01-box"
    "ch15-02-deref"
    "ch15-03-drop"
    "ch15-04-rc"
    "ch15-05-interior-mutability"
    "ch15-06-reference-cycles"
    "ch16-00-concurrency"
    "ch16-01-threads"
    "ch16-02-message-passing"
    "ch16-03-shared-state"
    "ch16-04-extensible-concurrency-sync-and-send"
    "ch17-00-oop"
    "ch17-01-what-is-oo"
    "ch17-02-trait-objects"
    "ch17-03-oo-design-patterns"
    "ch18-00-patterns"
    "ch18-01-all-the-places-for-patterns"
    "ch18-02-refutability"
    "ch18-03-pattern-syntax"
    "ch19-00-advanced-features"
    "ch19-01-unsafe-rust"
    "ch19-03-advanced-traits"
    "ch19-04-advanced-types"
    "ch19-05-advanced-functions-and-closures"
    "ch19-06-macros"
    "ch20-00-final-project-a-web-server"
    "ch20-01-single-threaded"
    "ch20-02-multithreaded"
    "ch20-03-graceful-shutdown"
    "ch21-00-appendix"
    "appendix-01-keywords"
    "appendix-02-operators"
    "appendix-03-derivable-traits"
    "appendix-04-useful-development-tools"
    "appendix-05-editions"
    "appendix-06-translation"
    "appendix-07-nightly-rust"
)

# Function to validate book references
check_references() {
    local metadata_file="$1"
    local exercise_path=$(dirname "$metadata_file")
    local exercise_name=$(basename "$exercise_path")
    local chapter_name=$(basename "$(dirname "$exercise_path")")
    
    TOTAL=$((TOTAL + 1))
    echo -n "  Checking $chapter_name/$exercise_name... "
    
    # Check primary chapter reference
    local primary_chapter=$(jq -r '.rust_book_refs.primary_chapter' "$metadata_file" 2>/dev/null)
    if [[ -z "$primary_chapter" ]] || [[ "$primary_chapter" == "null" ]]; then
        echo "❌ Missing primary chapter reference"
        FAILED=$((FAILED + 1))
        return 1
    fi
    
    # Get all URLs from specific_sections
    local urls=$(jq -r '.rust_book_refs.specific_sections[]?.url // empty' "$metadata_file" 2>/dev/null)
    local url_count=$(echo "$urls" | grep -c . || true)
    
    if [[ $url_count -eq 0 ]]; then
        echo "⚠️  No specific section URLs (primary: $primary_chapter)"
        WARNINGS=$((WARNINGS + 1))
        return 0
    fi
    
    # Check each URL
    local invalid_urls=0
    for url in $urls; do
        # Check different valid URL patterns
        if [[ "$url" =~ ^https://doc\.rust-lang\.org/book/([^.#]+)\.html ]]; then
            # Main Rust Book chapter (with or without fragment)
            local chapter_id="${BASH_REMATCH[1]}"
            if [[ ! " ${VALID_CHAPTERS[@]} " =~ " ${chapter_id} " ]]; then
                invalid_urls=$((invalid_urls + 1))
            fi
        elif [[ "$url" =~ ^https://doc\.rust-lang\.org/edition-guide/? ]]; then
            # Rust Edition Guide (valid)
            continue
        elif [[ "$url" =~ ^https://doc\.rust-lang\.org/std/ ]]; then
            # Standard library docs (valid)
            continue
        elif [[ "$url" =~ ^https://doc\.rust-lang\.org/reference/ ]]; then
            # Rust Reference (valid)
            continue
        elif [[ "$url" =~ ^https://doc\.rust-lang\.org/cargo/ ]]; then
            # Cargo Book (valid)
            continue
        else
            # URL doesn't match any expected pattern
            invalid_urls=$((invalid_urls + 1))
        fi
    done
    
    if [[ $invalid_urls -gt 0 ]]; then
        echo "❌ Found $invalid_urls invalid URLs"
        FAILED=$((FAILED + 1))
        return 1
    fi
    
    # Check chapter relevance tags
    local relevance_tags=$(jq -r '.rust_book_refs.specific_sections[]?.relevance // empty' "$metadata_file" 2>/dev/null)
    local valid_relevance=("core_concept" "supporting" "background" "advanced" "reference")
    local invalid_relevance=0
    
    for tag in $relevance_tags; do
        if [[ ! " ${valid_relevance[@]} " =~ " ${tag} " ]]; then
            invalid_relevance=$((invalid_relevance + 1))
        fi
    done
    
    if [[ $invalid_relevance -gt 0 ]]; then
        echo "❌ Invalid relevance tags"
        FAILED=$((FAILED + 1))
        return 1
    fi
    
    echo "✓ ($url_count URLs)"
}

# Find all metadata.json files
for metadata_file in exercises/ch*/ex*/metadata.json; do
    if [[ -f "$metadata_file" ]]; then
        check_references "$metadata_file"
    fi
done

echo ""
echo "📊 Reference Check Summary:"
echo "   Total exercises: $TOTAL"
echo "   Valid: $((TOTAL - FAILED - WARNINGS))"
echo "   Warnings: $WARNINGS"
echo "   Failed: $FAILED"
echo ""

if [[ $FAILED -eq 0 ]]; then
    if [[ $WARNINGS -gt 0 ]]; then
        echo "✅ All Rust Book references are valid (with $WARNINGS warnings)"
    else
        echo "✅ All Rust Book references are valid!"
    fi
    exit 0
else
    echo "❌ Some Rust Book references are invalid"
    exit 1
fi