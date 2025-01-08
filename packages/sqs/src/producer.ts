import {
  PublishBatchCommand,
  PublishCommand,
  SNSClient,
} from '@aws-sdk/client-sns'
import type { Message, ProducerOptions } from './sqs.types.js'

const requiredOptions: Array<keyof ProducerOptions> = ['topicArn']

export class Producer {
  static create: (options: ProducerOptions) => Producer
  topicArn: string
  // batchSize: number;
  sns: SNSClient
  region?: string

  constructor(options: ProducerOptions) {
    this.validate(options)
    this.topicArn = options.topicArn
    this.sns =
      options.sns ||
      new SNSClient({
        ...options,
        region: options.region || process.env.AWS_REGION || 'eu-central-1',
      })
  }

  send(message: Message) {
    return this.sns.send(
      new PublishCommand({
        TopicArn: this.topicArn,
        Message: message.body,
        MessageDeduplicationId: message.deduplicationId,
        MessageAttributes: message.messageAttributes,
      }),
    )
  }

  sendBatch(message: Message[]) {
    return this.sns.send(
      new PublishBatchCommand({
        TopicArn: this.topicArn,
        PublishBatchRequestEntries: message.map(message => ({
          Id: message.id,
          Message: message.body,
          MessageDeduplicationId: message.deduplicationId,
          MessageAttributes: message.messageAttributes,
        })),
      }),
    )
  }

  /**
   * Validate the producer options.
   * @param options - The producer options to validate.
   * @throws Error if any required options are missing or invalid.
   */
  private validate(options: ProducerOptions): void {
    for (const option of requiredOptions) {
      if (!options[option]) {
        throw new Error(`Missing SNS Producer option [${option}].`)
      }
    }
    // if (options.batchSize > 10 || options.batchSize < 1) {
    //   throw new Error('SQS batchSize option must be between 1 and 10.');
    // }
  }
}

Producer.create = (options: ProducerOptions): Producer => {
  return new Producer(options)
}
