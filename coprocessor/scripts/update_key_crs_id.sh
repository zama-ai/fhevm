#!/bin/sh

# Define file paths
LOGS_FILE="/app/logs_keygen/key_crs_gen.log"
CONFIG_FILE="/app/gateway/config/gateway.toml"

# Extract the first keygen_response request_id
KEYGEN_ID=$(awk '/keygen_response/ {found=1} found && /"request_id"/ {print $2; exit}' "$LOGS_FILE" | tr -d '",')

# Extract the first crs_gen_response request_id
CRS_ID=$(awk '/crs_gen_response/ {found=1} found && /"request_id"/ {print $2; exit}' "$LOGS_FILE" | tr -d '",')

# Validate extracted values
if [ -z "$KEYGEN_ID" ] || [ -z "$CRS_ID" ]; then
  echo "Error: Unable to extract keygen or crs IDs from logs."
  exit 1
fi


# File paths
ORIGINAL_FILE="/app/gateway/config/gateway.toml"
TEMP_FILE="/tmp/gateway.toml"

# Write the modified content to a temporary file
cp "$ORIGINAL_FILE" "$TEMP_FILE"
sed -i "s/^key_id = \".*\"/key_id = \"$KEYGEN_ID\"/" "$TEMP_FILE"
sed -i "s/^crs_id = \".*\"/crs_id = \"$CRS_ID\"/" "$TEMP_FILE"

# Overwrite the original file by redirecting
cat "$TEMP_FILE" > "$ORIGINAL_FILE"
rm -f "$TEMP_FILE"



echo "Updated gateway.toml with key_id=$KEYGEN_ID and crs_id=$CRS_ID."
