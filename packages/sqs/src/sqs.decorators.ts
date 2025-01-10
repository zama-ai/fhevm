import { SetMetadata } from '@nestjs/common'
import {
  SQS_CONSUMER_EVENT_HANDLER,
  SQS_CONSUMER_METHOD,
} from './sqs.constants.js'

export const SqsMessageHandler = (name: string, batch?: boolean) =>
  SetMetadata(SQS_CONSUMER_METHOD, { name, batch })

export const SqsConsumerEventHandler = (name: string, eventName: string) =>
  SetMetadata(SQS_CONSUMER_EVENT_HANDLER, { name, eventName })
