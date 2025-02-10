import { SharedModule } from '#shared/shared.module.js'
import { Module } from '@nestjs/common'
import { SnsProducer } from './sns-producer.js'

@Module({
  imports: [SharedModule],
  providers: [SnsProducer],
})
export class SNSProducerModule {}
