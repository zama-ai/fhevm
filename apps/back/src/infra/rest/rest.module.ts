import { KeyUrlService } from '#httpz/domain/service/key-url.service.js'
import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { Module } from '@nestjs/common'
import { ConfigKeyUrlService } from './adapters/config-key-url.service.js'
import { HttpzController } from './httpz.controller.js'
import { InputProof } from '#httpz/use-cases/input-proof.use-case.js'
import { SharedModule } from '#shared/shared.module.js'

@Module({
  imports: [SharedModule],
  controllers: [HttpzController],
  providers: [
    GetKeyUrl,
    {
      provide: KeyUrlService,
      useClass: ConfigKeyUrlService,
    },
    InputProof,
  ],
})
export class RestModule {}
