import { KEY_URL_SERVICE } from '#httpz/domain/service/key-url.service.js'
import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { Module } from '@nestjs/common'
import { ConfigKeyUrlService } from './adapters/config-key-url.service.js'
import { HttpzController } from './httpz.controller.js'
import * as uc from '#httpz/use-cases/index.js'
import { SharedModule } from '#shared/shared.module.js'
import { DappsModule } from '#dapps/infra/dapps.module.js'

@Module({
  imports: [SharedModule, DappsModule],
  controllers: [HttpzController],
  providers: [
    GetKeyUrl,
    {
      provide: KEY_URL_SERVICE,
      useClass: ConfigKeyUrlService,
    },
    uc.InputProof,
    uc.GetKeyUrl,
  ],
})
export class RestModule {}
