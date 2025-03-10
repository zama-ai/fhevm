import { registerAs } from '@nestjs/config'

export default registerAs('aws', () => ({
  endpoint: process.env.AWS_ENDPOINT,
  region: process.env.AWS_REGION,
  back: {
    queueUrl: process.env.AWS_BACK_QUEUE_URL,
  },
  orchestrator: {
    queueUrl: process.env.AWS_ORCHESTRATOR_QUEUE_URL,
  },
}))
