#!/bin/sh

echo "Initializing localstack"

echo "Creating main topic"
awslocal sns create-topic \
  --name console-topic.fifo \
  --attributes "FifoTopic=true,ContentBasedDeduplication=true"

echo "Creating Orchestrator queue"
awslocal sqs create-queue \
  --queue-name orchestrator-queue.fifo \
  --attributes "FifoQueue=true"
awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic.fifo" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:orchestrator-queue.fifo"

echo "Creating Web3 queue"
awslocal sqs create-queue \
  --queue-name web3-queue.fifo \
  --attributes "FifoQueue=true"
awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic.fifo" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:web3-queue.fifo"

echo "Creating Email queue"
awslocal sqs create-queue \
  --queue-name email-queue.fifo \
  --attributes "FifoQueue=true"
awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic.fifo" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:email-queue.fifo"