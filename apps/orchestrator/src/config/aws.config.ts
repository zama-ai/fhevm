import { registerAs } from '@nestjs/config';

export default registerAs('aws', () => ({
  endpoint: process.env.AWS_ENDPOINT,
  queueUrl: process.env.AWS_QUEUE_URL,
  region: process.env.AWS_REGION,
}));
