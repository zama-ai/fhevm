import { KeyUrlService } from '#httpz/domain/service/key-url.service.js'
import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { Module } from '@nestjs/common'
import { ConfigKeyUrlService } from './adapters/config-key-url.service.js'
import { HttpzController } from './httpz.controller.js'

@Module({
  controllers: [HttpzController],
  providers: [
    GetKeyUrl,
    {
      provide: KeyUrlService,
      useClass: ConfigKeyUrlService,
    },
  ],
})
export class RestModule {}
