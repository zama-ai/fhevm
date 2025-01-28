#!/bin/bash

# setup-config.sh
# Script to set up local configuration

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Setting up local configuration...${NC}"

# Create config directory if it doesn't exist
mkdir -p config

# Check if local.yaml already exists
if [ -f config/local.yaml ]; then
    echo -e "${YELLOW}Warning: config/local.yaml already exists${NC}"
    read -p "Do you want to overwrite it? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Keeping existing configuration"
        exit 0
    fi
fi

# Copy example configuration
cp config/local.yaml.example config/local.yaml

echo -e "${GREEN}Configuration set up successfully!${NC}"
echo "You can now edit config/local.yaml with your local settings"
echo -e "${YELLOW}Don't forget to:${NC}"
echo "1. Update the contract addresses"
echo "2. Set the correct WebSocket URL for your local node"
echo "3. Adjust logging settings if needed"