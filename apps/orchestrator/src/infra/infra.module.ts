import { Module } from '@nestjs/common'
import { SnsProducer } from './sns.producer.js'
import { PUBSUB } from '#constants.js'
import { PubSub } from 'utils'
import { SQSConsumer } from './sqs.consumer.js'

@Module({
  providers: [
    {
      provide: PUBSUB,
      useClass: PubSub,
    },
    SnsProducer,
    SQSConsumer,
  ],
})
export class InfraModule {}
