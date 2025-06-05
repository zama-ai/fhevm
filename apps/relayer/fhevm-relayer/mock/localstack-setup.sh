#!/bin/sh
echo "installing jq"
apt-get -y install jq

echo "Initializing localstack"

echo "Creating Orchestrator queue"
awslocal sqs create-queue --queue-name orchestrator-dlq
awslocal sqs create-queue --queue-name orchestrator-queue \
  --attributes '{"RedrivePolicy":"{\"deadLetterTargetArn\":\"arn:aws:sqs:eu-central-1:000000000000:orchestrator-dlq\",\"maxReceiveCount\":\"1\"}"}'

echo "Creating Relayer queue"
awslocal sqs create-queue --queue-name relayer-dlq
awslocal sqs create-queue --queue-name relayer-queue \
  --attributes '{"RedrivePolicy":"{\"deadLetterTargetArn\":\"arn:aws:sqs:eu-central-1:000000000000:relayer-dlq\",\"maxReceiveCount\":\"1\"}"}'

# Used in docker compose to check the health of the container
 touch /tmp/done
