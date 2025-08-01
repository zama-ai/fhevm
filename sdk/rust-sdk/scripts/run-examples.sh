#!/bin/bash

# Array of examples to run
examples=(
    "minimal-eip712-signing"
    "minimal-encrypted-input"
    "minimal-sdk-setup"
    "minimal-user-keys-generation"
    "minimal-user-decryption-request"
    "minimal-user-decryption-response"
    "minimal-public-decryption-request"
    "minimal-public-decryption-response"
)

# Array to store error messages
errors=()
failed_examples=()

# Function to display banner
display_banner() {
    local example_name="$1"
    echo "=================================="
    echo "Running example: $example_name"
    echo "=================================="
}

# Run all examples
for example in "${examples[@]}"; do
    display_banner "$example"

    # Create temporary file to capture output
    temp_error_file=$(mktemp)

    # Run the example, capture exit code and display output
    cargo run --example "$example" &> "$temp_error_file"
    exit_code="$?"
    cat "$temp_error_file"

    # Run the example and capture stderr while showing stdout
    if [ "$exit_code" -eq 0 ]; then
        echo ""
        echo "----------------------------------"
        echo "✅ Example '$example' completed successfully"
        echo "----------------------------------"
    else
        # Read the actual error message from cargo
        error_output=$(cat "$temp_error_file")
        errors+=("Example '$example' failed with error: $error_output")
        failed_examples+=("$example")

        echo ""
        echo "----------------------------------"
        echo "❌ Example '$example' failed"
        echo "----------------------------------"
    fi

    # Clean up temporary file
    rm -f "$temp_error_file"

    echo ""  # Add blank line for readability between examples
done

# Display summary
echo "=================================="
echo "EXECUTION SUMMARY"
echo "=================================="

if [ ${#failed_examples[@]} -eq 0 ]; then
    echo "✅ All examples completed successfully!"
    exit 0
else
    echo "❌ ${#failed_examples[@]} example(s) failed:"

    for i in "${!failed_examples[@]}"; do
        echo "  - ${failed_examples[$i]}"
    done

    echo ""
    echo "Error details:"
    for error in "${errors[@]}"; do
        echo "  • $error"
        echo ""
    done

    exit 1
fi
