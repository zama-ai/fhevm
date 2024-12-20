.PHONY: publish-app-deployment-requested
publish-app-deployment-requested:
	aws --endpoint=http://localhost:4566 sns publish \
		--topic-arn 'arn:aws:sns:eu-central-1:000000000000:console-topic' \
		--region eu-central-1 \
		--message '{"_tag": "Event", "type": "app-deployment.requested", "payload": {"applicationId": "test-app", "deploymentId": "depl-id", "address": "0x12345", "chainId": "1"}}'

.PHONY: publish-app-deployment-discover-sc
publish-app-deployment-discover-sc:
	aws --endpoint=http://localhost:4566 sns publish \
		--topic-arn 'arn:aws:sns:eu-central-1:000000000000:console-topic' \
		--region eu-central-1 \
		--message '{"_tag": "Command", "type": "app-deployment.discover-sc", "payload": {"applicationId": "test-app", "deploymentId": "depl-id", "address": "0x278a72ccffee5dc758c1b573ca71f377609e39af", "chainId": "11155111"}}'
