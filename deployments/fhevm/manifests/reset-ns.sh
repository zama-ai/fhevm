#!/bin/bash

NAMESPACE=$1

if [ -z "$NAMESPACE" ]; then
    echo "Please provide a namespace name"
    exit 1
fi

kubectl delete namespace "$NAMESPACE"

# Create namespace if it doesn't exist
kubectl create namespace "$NAMESPACE" --dry-run=client -o yaml | kubectl apply -f -

# Apply registry secret
kubectl apply -f '/Users/amina/zama/docs/.secrets/registry-secret.yaml' -n "$NAMESPACE"