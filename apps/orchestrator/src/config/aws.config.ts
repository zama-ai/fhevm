import { registerAs } from '@nestjs/config'

export default registerAs('aws', () => ({
  endpoint: process.env.AWS_ENDPOINT,
  back: {
    queueUrl: process.env.AWS_BACK_QUEUE_URL,
  },
  orchestrator: {
    queueUrl: process.env.AWS_ORCHESTRATOR_QUEUE_URL,
  },
  relayer: {
    queueUrl: process.env.AWS_RELAYER_QUEUE_URL,
  },
  web3: {
    queueUrl: process.env.AWS_WEB3_QUEUE_URL,
  },
  email: {
    queueUrl: process.env.AWS_EMAIL_QUEUE_URL,
  },
  region: process.env.AWS_REGION,
}))
