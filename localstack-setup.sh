#!/bin/sh
echo "installing jq"
apt-get -y install jq

echo "Initializing localstack"

echo "Creating Back queue"
awslocal sqs create-queue --queue-name back-dlq
awslocal sqs create-queue --queue-name back-queue \
  --attributes '{"RedrivePolicy":"{\"deadLetterTargetArn\":\"arn:aws:sqs:eu-central-1:000000000000:back-dlq\",\"maxReceiveCount\":\"1\"}"}'

echo "Creating Orchestrator queue"
awslocal sqs create-queue --queue-name orchestrator-dlq
awslocal sqs create-queue --queue-name orchestrator-queue \
  --attributes '{"RedrivePolicy":"{\"deadLetterTargetArn\":\"arn:aws:sqs:eu-central-1:000000000000:orchestrator-dlq\",\"maxReceiveCount\":\"1\"}"}'

echo "Creating Web3 queue"
awslocal sqs create-queue --queue-name web3-dlq
awslocal sqs create-queue --queue-name web3-queue \
  --attributes '{"RedrivePolicy":"{\"deadLetterTargetArn\":\"arn:aws:sqs:eu-central-1:000000000000:web3-dlq\",\"maxReceiveCount\":\"1\"}"}'

echo "Creating Relayer queue"
awslocal sqs create-queue --queue-name relayer-dlq
awslocal sqs create-queue --queue-name relayer-queue \
  --attributes '{"RedrivePolicy":"{\"deadLetterTargetArn\":\"arn:aws:sqs:eu-central-1:000000000000:relayer-dlq\",\"maxReceiveCount\":\"1\"}"}'

echo "Creating Email queue"
awslocal sqs create-queue --queue-name email-dlq 
awslocal sqs create-queue --queue-name email-queue \
  --attributes '{"RedrivePolicy":"{\"deadLetterTargetArn\":\"arn:aws:sqs:eu-central-1:000000000000:email-dlq\",\"maxReceiveCount\":\"1\"}"}'

echo "Creating relayer queue"
awslocal sqs create-queue --queue-name relayer-queue
