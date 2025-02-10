import { Module } from '@nestjs/common'
import { SnsProducer } from './sns.producer.js'
import { SQSConsumer } from './sqs.consumer.js'
import { SharedModule } from '#shared/shared.module.js'

@Module({
  imports: [SharedModule],
  providers: [SnsProducer, SQSConsumer],
})
export class InfraModule {}
