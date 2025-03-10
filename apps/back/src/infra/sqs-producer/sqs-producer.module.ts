import { SharedModule } from '#shared/shared.module.js'
import { Module } from '@nestjs/common'
import { SqsProducer } from './sqs.producer.js'

@Module({
  imports: [SharedModule],
  providers: [SqsProducer],
})
export class SqsProducerModule {}
