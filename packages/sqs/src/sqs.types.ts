import type { MessageAttributeValue, SNSClient } from '@aws-sdk/client-sns'
import { LoggerService, ModuleMetadata, Type } from '@nestjs/common'
import type { Consumer, ConsumerOptions, StopOptions } from 'sqs-consumer'

export interface ProducerOptions {
  /**
   * The ARN of the topic to send messages to.
   */
  topicArn: string

  /**
   * The SNS Client to use. If not provided, a new client will be created.
   */
  sns?: SNSClient

  /**
   * The AWS region to use. If not provided, the region will be determined
   * from the `AWS_REGION` environment variable or will default to `eu-central-1.
   */
  region?: string
}

export interface Message {
  /**
   * An identifier for the message. This must be unique within
   * the batch of messages.
   */
  id: string
  /**
   * The messages contents.
   */
  body: string
  /**
   * This parameter applies only to FIFO (first-in-first-out) queues.
   * When set messages that belong to the same message group are processed
   * in a FIFO manner
   */
  groupId?: string
  /**
   * This parameter applies only to FIFO (first-in-first-out) queues.
   * The token used for deduplication of messages within a 5-minute minimum
   * deduplication interval. If a message with a particular id is sent successfully,
   * subsequent messages with the same id are accepted
   * successfully but aren't delivered.
   */
  deduplicationId?: string
  /**
   * The length of time, in seconds, for which to delay a specific message.
   * Valid values: 0 to 900. Maximum: 15 minutes.
   */
  delaySeconds?: number
  /**
   * Each message attribute consists of a Name, Type, and Value. For more
   * information, see [Amazon SQS message attributes](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/sqs-message-attributes.html).
   */
  messageAttributes?: { [key: string]: MessageAttributeValue }
}

export type SqsConsumerOptions = Omit<
  ConsumerOptions,
  'handleMessage' | 'handleMessageBatch'
> & {
  name: string
  stopOptions?: StopOptions
}

export type SnsProducerOptions = ProducerOptions & {
  name: string
}

export type SqsConsumerMapValues = {
  instance: Consumer
  stopOptions: StopOptions
}

export interface SqsOptions {
  consumers?: SqsConsumerOptions[]
  producers?: SnsProducerOptions[]
  logger?: LoggerService
  globalStopOptions?: StopOptions
}

export interface SqsModuleOptionsFactory {
  createOptions(): Promise<SqsOptions> | SqsOptions
}

export interface SqsModuleAsyncOptions extends Pick<ModuleMetadata, 'imports'> {
  inject?: any[]
  useFactory?: (...args: any[]) => Promise<SqsOptions> | SqsOptions
  useExisting?: Type<SqsModuleOptionsFactory>
  useClass?: Type<SqsModuleOptionsFactory>
}

export interface SqsMessageHanlderMeta {
  name: string
  batch?: boolean
}

export interface SqsConsumerEventHandlerMeta {
  name: string
  eventName: string
}
