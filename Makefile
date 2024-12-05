.PHONY: publish-app-deployment-requested
publish-app-deployment-requested:
	aws --endpoint=http://localhost:4566 sns publish \
		--topic-arn 'arn:aws:sns:eu-central-1:000000000000:console-topic.fifo' \
		--region eu-central-1 \
		--message-group-id app-deployment \
		--message '{"_tag": "Event", "type": "app-deployment.requested", "payload": {"applicationId": "test-app", "address": "0x12345", "chainId": "1"}}'