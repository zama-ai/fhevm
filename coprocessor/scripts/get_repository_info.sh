#!/usr/bin/env bash

REPO_NAME=$1
REPO_PATH=$2

if [ ! -d "$REPO_PATH" ]; then
    echo "######################################################"
    echo "WARNING: $REPO_NAME does not exist or is not a directory"
    echo "Given path: $REPO_PATH"
    echo "######################################################"
    exit
fi

if [ ! -d "$REPO_PATH/.git" ]; then
    echo "Error: $REPO_PATH is not a Git repository"
    exit 1
fi

cd $REPO_PATH

BRANCH=$(git rev-parse --abbrev-ref HEAD)
TAG=$(git describe --tags --exact-match 2>/dev/null)
COMMIT=$(git rev-parse HEAD | cut -c 1-8)

echo "$REPO_NAME --- branch: $BRANCH | tag: $TAG | commit: $COMMIT | path: $REPO_PATH"
