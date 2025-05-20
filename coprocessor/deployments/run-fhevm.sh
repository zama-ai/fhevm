#!/bin/bash

set -o errexit

# run layer 1 and deploy sc
docker compose --env-file ./config/.env.staging.layer1 -p zama -f layer1-docker-compose.yml up -d

# run layer 2 and deploy sc
docker compose --env-file ./config/.env.staging.layer2 -p zama -f layer2-docker-compose.yml up -d

# run coprocessor
docker compose --env-file ./config/.env.staging.coprocessor -p zama -f coprocessor-docker-compose.yml up -d