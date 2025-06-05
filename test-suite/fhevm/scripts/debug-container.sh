#!/bin/bash

CONTAINER_NAME=$1

if [ -z "$CONTAINER_NAME" ]; then
    echo "Usage: $0 <container-name>"
    exit 1
fi

echo "Fetching logs for $CONTAINER_NAME..."
docker logs $CONTAINER_NAME

echo -e "\nContainer status:"
docker inspect $CONTAINER_NAME --format='{{.State.Status}}'

echo -e "\nExit code:"
docker inspect $CONTAINER_NAME --format='{{.State.ExitCode}}'

echo -e "\nError (if any):"
docker inspect $CONTAINER_NAME --format='{{.State.Error}}'

echo -e "\nStarted at:"
docker inspect $CONTAINER_NAME --format='{{.State.StartedAt}}'

echo -e "\nFinished at:"
docker inspect $CONTAINER_NAME --format='{{.State.FinishedAt}}'
