import { SharedModule } from '#shared/shared.module.js'
import { Global, Module } from '@nestjs/common'
import { SqsProducer } from './sqs.producer.js'
import { PRODUCER } from '#constants.js'

@Global()
@Module({
  imports: [SharedModule],
  providers: [
    {
      provide: PRODUCER,
      useClass: SqsProducer,
    },
  ],
  exports: [PRODUCER],
})
export class SqsProducerModule {}
