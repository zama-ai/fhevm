#!/bin/sh
set -euo pipefail

# Validate that required environment variables are set
if [ -z "$REACT_APP_DEV_PORTAL_API_SERVER" ]; then
  echo "❌ Error: The REACT_APP_DEV_PORTAL_API_SERVER environment variable is not set."
  exit 1
fi
if [ -z "$REACT_APP_AUTH0_DOMAIN" ]; then
  echo "❌ Error: The REACT_APP_AUTH0_DOMAIN environment variable is not set."
  exit 1
fi
if [ -z "$REACT_APP_AUTH0_CLIENT_ID" ]; then
  echo "❌ Error: The REACT_APP_AUTH0_CLIENT_ID environment variable is not set."
  exit 1
fi
# Note: REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID is optional
if [ -z "$REACT_APP_STRIPE_PUBLISHABLE_KEY" ]; then
  echo "❌ Error: The REACT_APP_STRIPE_PUBLISHABLE_KEY environment variable is not set."
  exit 1
fi
if [ -z "$REACT_APP_STRIPE_MANAGEMENT_URL" ]; then
  echo "❌ Error: The REACT_APP_STRIPE_MANAGEMENT_URL environment variable is not set."
  exit 1
fi

# The path to the output configuration file
CONFIG_FILE="/usr/share/nginx/html/env-config.js"

# Create the configuration file
# Using a temporary file ensures atomic write and helps with permissions
TMP_FILE=$(mktemp)

# Create the configuration file
cat <<EOF > "$TMP_FILE"
window.VITE_CONFIG = {
  REACT_APP_DEV_PORTAL_API_SERVER: "${REACT_APP_DEV_PORTAL_API_SERVER}",
  REACT_APP_AUTH0_DOMAIN: "${REACT_APP_AUTH0_DOMAIN}",
  REACT_APP_AUTH0_CLIENT_ID: "${REACT_APP_AUTH0_CLIENT_ID}",
  REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID: "${REACT_APP_MOESIF_PUBLISHABLE_APPLICATION_ID}",
  REACT_APP_STRIPE_PUBLISHABLE_KEY: "${REACT_APP_STRIPE_PUBLISHABLE_KEY}",
  REACT_APP_STRIPE_MANAGEMENT_URL: "${REACT_APP_STRIPE_MANAGEMENT_URL}"
};
EOF

# Set secure permissions and move the file into place
chmod 644 "$TMP_FILE"
mv "$TMP_FILE" "$CONFIG_FILE"

echo "✅ env-config.js created successfully at ${CONFIG_FILE}"

# Start the web server (in this case, Nginx)
# `exec "$@"` will run the command passed to the script.
# In our Dockerfile, this will be `nginx -g 'daemon off;'`
exec "$@"