#!/bin/sh
echo "installing jq"
apt-get -y install jq

echo "Initializing localstack"

echo "Creating main topic"
awslocal sns create-topic \
  --name console-topic \
  --attributes "FifoTopic=false,ContentBasedDeduplication=true"

echo "Creating Back queue"
awslocal sqs create-queue \
  --queue-name back-queue
BACK_SUBSCRIPTION_ARN="$(awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:back-queue" | jq -r '.SubscriptionArn')"

echo "Back subscription created $BACK_SUBSCRIPTION_ARN"
awslocal sns set-subscription-attributes \
  --subscription-arn $BACK_SUBSCRIPTION_ARN \
  --attribute-name "FilterPolicyScope" \
  --attribute-value "MessageBody"
awslocal sns set-subscription-attributes \
  --subscription-arn $BACK_SUBSCRIPTION_ARN \
  --attribute-name "FilterPolicy" \
  --attribute-value '{"type": [{"prefix": "back:"}]}'

echo "Creating Orchestrator queue"
awslocal sqs create-queue \
  --queue-name orchestrator-queue
ORCH_SUBSCRIPTION_ARN="$(awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:orchestrator-queue" | jq -r '.SubscriptionArn')"

echo "Orchestrator subscription created $ORCH_SUBSCRIPTION_ARN"
awslocal sns set-subscription-attributes \
  --subscription-arn $ORCH_SUBSCRIPTION_ARN \
  --attribute-name "FilterPolicyScope" \
  --attribute-value "MessageAttributes"
awslocal sns set-subscription-attributes \
  --subscription-arn $ORCH_SUBSCRIPTION_ARN \
  --attribute-name "FilterPolicy" \
  --attribute-value '{"Sender": [{"anything-but": "orch"}]}'

echo "Creating Web3 queue"
awslocal sqs create-queue \
  --queue-name web3-queue
WEB3_SUBSCRIPTION_ARN="$(awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:web3-queue" | jq -r '.SubscriptionArn')"

echo "Web3 Subscription created: $WEB3_SUBSCRIPTION_ARN"
awslocal sns set-subscription-attributes \
  --subscription-arn $WEB3_SUBSCRIPTION_ARN \
  --attribute-name "FilterPolicyScope" \
  --attribute-value "MessageBody"
awslocal sns set-subscription-attributes \
  --subscription-arn $WEB3_SUBSCRIPTION_ARN \
  --attribute-name "FilterPolicy" \
  --attribute-value '{"type": [{"prefix": "web3:"}]}'

echo "Creating Email queue"
awslocal sqs create-queue \
  --queue-name email-queue 
EMAIL_SUBSCRIPTION_ARN="$(awslocal sns subscribe \
  --topic-arn "arn:aws:sns:eu-central-1:000000000000:console-topic" \
  --protocol sqs \
  --notification-endpoint "arn:aws:sqs:eu-central-1:000000000000:email-queue" | jq -r '.SubscriptionArn')"

awslocal sns set-subscription-attributes \
  --subscription-arn $EMAIL_SUBSCRIPTION_ARN \
  --attribute-name "FilterPolicyScope" \
  --attribute-value "MessageBody"
awslocal sns set-subscription-attributes \
  --subscription-arn $EMAIL_SUBSCRIPTION_ARN \
  --attribute-name "FilterPolicy" \
  --attribute-value '{"type": [{"prefix": "email:"}]}'
