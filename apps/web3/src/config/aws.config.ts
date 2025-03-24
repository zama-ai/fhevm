import { registerAs } from '@nestjs/config'

export default registerAs('aws', () => ({
  endpoint: process.env.AWS_ENDPOINT,
  region: process.env.AWS_REGION,
  web3: {
    queueUrl: process.env.AWS_WEB3_QUEUE_URL,
  },
  orchestrator: {
    queueUrl: process.env.AWS_ORCHESTRATOR_QUEUE_URL,
  },
}))
