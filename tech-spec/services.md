# Services
<!-- TODO: update -->
## /apps/front
### Purpose
A bunch of static assets (html, css, js & svg) loaded by the browser to interact with the `/back` and displays a nice ui to the user.
### Interfaces:
- web URL (publicly hosted on CDN)
### Tech stack
React, Apollo-Client
### code & readme
[/apps/front/README.md](/apps/front/README.md)
## /apps/back
### Purposes:
- provide data to to the frontend
- authentifies users
- stores users & apps details
- stores usages data (& payment)
- authentifies & allow reencryptions ([conversation in progress](https://www.notion.so/zamaai/RFC-handle-async-reencryption-requests-1a45a7358d5e8028b07af0246cdf673a?pvs=4))
### Interfaces:
- HTTP /graphql
  - POST https://console-api.zws.cloud/graphql
  - WS wss://console-api.zws.cloud/graphql
- postgresql
- redis
* can read a SQS queue and publish on a SNS topic
### Tech stack
nodejs, nestjs, apollo-server, prisma, postgresql
### code & readme
[/apps/back/README.md](/apps/back/README.md)
## /apps/relayer
### Purpose
Interact with Gateway Chain on one side and Clients through the Relayer SDK on the other.
- Get configuration per host chain
	- Public Key material
	- Contract addresses
	- ...
- Submit fresh encrypted values
- Submit user decryption requests (re-encryption)
- Submit public decryption requests
- Fulfill public decryption oracle calls
- Encrypt values (for users that don't want to )
### Interfaces:
- HTTP
### Tech stack
Rust
### code & readme

[/apps/relayer/README.md](/apps/relayer/README.md)