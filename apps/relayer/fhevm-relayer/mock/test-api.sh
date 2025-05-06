#!/usr/bin/env bash

# Combined script to test both user-decrypt and input-proof endpoints
# Exit with non-zero code if any test fails

set -e

echo "===== API Test Suite ====="
failed=0

# Function to run a test and validate its output
run_test() {
  local test_name=$1
  local command=$2
  local endpoint=$3
  local pattern=$4
  
  echo "Testing $test_name endpoint: $endpoint"
  
  # Run the test and capture output
  echo "$ $command"
  RESPONSE=$(eval $command | tail -n 1)
  echo "Received: $RESPONSE"
  
  # Validate response against pattern
  if echo "$RESPONSE" | grep -q "$pattern"; then
    echo "✅ $test_name test passed!"
  else
    echo "❌ $test_name test failed!"
    echo "Expected pattern: $pattern"
    echo "Got: $RESPONSE"
    return 1
  fi
  
  return 0
}

# Test user-decrypt endpoint
if ! run_test "user-decrypt" "make run-test-user-decrypt" \
    "http://localhost:3000/v1/user-decrypt" \
    '{"response":\[.*\]}'; then
  failed=1
fi

echo ""  # Add spacing between tests

# Test input-proof endpoint
if ! run_test "input-proof" "make run-test-input-proof" \
    "http://localhost:3000/v1/input-proof" \
    '{"response":{"handles":\[.*\],"signatures":\[.*\]}}'; then
  failed=1
fi

# Report final status
echo ""
if [ $failed -eq 0 ]; then
  echo "✅ All tests passed!"
  exit 0
else
  echo "❌ One or more tests failed!"
  exit 1
fi