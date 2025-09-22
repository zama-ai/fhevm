#!/usr/bin/env bash

set -e

# Install dependencies
echo "Installing dependencies..."
echo ""
npm install

# Build the application
echo "Building the application..."
echo ""
npm run build

# Install serve globally
npm install -g serve

# Start the server
echo "Starting the app at port 4000..."
echo ""
exec serve -p 4000 -s dist
