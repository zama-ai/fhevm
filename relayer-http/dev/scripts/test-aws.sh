#!/usr/bin/env bash
# Runs the `aws_kms_emulator` tests (ignored by a plain test run) against Floci, an AWS emulator, started here and
# stopped on exit. The AWS settings reach the relayer as in a deployment: region, endpoint and credentials from the
# AWS environment, the credentials the IRSA way (a role ARN and a web identity token file, exchanged with STS).
# moto cannot replace Floci: its KMS Sign ignores MessageType=DIGEST and hashes the digest again.
# Usage: test-aws.sh (FLOCI_PORT=4566 by default). Used by `make -C dev test-aws` and CI.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
IMAGE="floci/floci:2.3.0"
PORT="${FLOCI_PORT:-4566}"
NAME="relayer-http-floci-$$"
URL="http://127.0.0.1:$PORT"

docker run --rm -d --name "$NAME" -p "127.0.0.1:$PORT:4566" -e FLOCI_SERVICES_UI_ENABLED=false "$IMAGE" >/dev/null
trap 'docker stop "$NAME" >/dev/null' EXIT
for _ in $(seq 1 50); do
  curl -sf "$URL/_localstack/health" >/dev/null && break
  sleep 0.2
done
curl -sf "$URL/_localstack/health" >/dev/null || { echo "Floci did not start on port $PORT"; exit 1; }

# IAM query API call; Floci checks no signature, any credential works.
iam() {
  curl -sf -X POST "$URL/" \
    -H 'Authorization: AWS4-HMAC-SHA256 Credential=test/20260101/us-east-1/iam/aws4_request, SignedHeaders=host, Signature=00' \
    --data-urlencode Version=2010-05-08 "$@"
}

# The role IRSA would assume, trusting web identities.
POLICY='{"Version":"2012-10-17","Statement":[{"Effect":"Allow","Principal":{"Federated":"arn:aws:iam::000000000000:oidc-provider/test"},"Action":"sts:AssumeRoleWithWebIdentity"}]}'
ROLE_ARN="$(iam --data-urlencode Action=CreateRole --data-urlencode RoleName=relayer-http \
  --data-urlencode "AssumeRolePolicyDocument=$POLICY" | sed -n 's|.*<Arn>\([^<]*\)</Arn>.*|\1|p')"
[ -n "$ROLE_ARN" ] || { echo "Floci did not create the IAM role"; exit 1; }

# IRSA on EKS sets these; Floci's STS accepts any token for an existing role.
TOKEN_FILE="$ROOT/target/aws-web-identity-token"
mkdir -p "$ROOT/target" && echo "test-token" > "$TOKEN_FILE"
export AWS_ROLE_ARN="$ROLE_ARN"
export AWS_WEB_IDENTITY_TOKEN_FILE="$TOKEN_FILE"
export AWS_REGION="us-east-1"
export AWS_DEFAULT_REGION="us-east-1"
export AWS_ENDPOINT_URL="$URL"

cd "$ROOT" && cargo nextest run -p relayer-http --run-ignored only -E 'test(/aws_kms_emulator/)'
