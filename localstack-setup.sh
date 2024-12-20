#!/bin/sh

echo "Initializing localstack"

echo "Creating main topic"
awslocal sns create-topic \
  --name console-topic \
  --attributes "FifoTopic=false,ContentBasedDeduplication=true"

echo "Creating Back queue"
awslocal sqs create-queue \
  --queue-name back-queue
awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:back-queue"

echo "Creating Orchestrator queue"
awslocal sqs create-queue \
  --queue-name orchestrator-queue
awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:orchestrator-queue"

echo "Creating Web3 queue"
awslocal sqs create-queue \
  --queue-name web3-queue
awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:web3-queue"

echo "Creating Email queue"
awslocal sqs create-queue \
  --queue-name email-queue 
awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:email-queue"
