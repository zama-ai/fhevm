import { KEY_URL_SERVICE } from '#httpz/domain/service/key-url.service.js'
import { GetKeyUrl } from '#httpz/use-cases/get-key-url.use-case.js'
import { Module } from '@nestjs/common'
import { ConfigKeyUrlService } from './adapters/config-key-url.service.js'
import { HttpzController } from './httpz.controller.js'
import * as uc from '#httpz/use-cases/index.js'
import { SharedModule } from '#shared/shared.module.js'
import { DappsModule } from '#dapps/infra/dapps.module.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { SyncInstances } from '#shared/use-cases/sync-instances.use-case.js'
import {
  API_KEY_ALLOWS_REQUEST,
  IApiKeyAllowsRequest,
} from '#dapps/use-cases/api-key-allows-request.use-case.js'
import { PRODUCER } from '#constants.js'

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
    {
      provide: uc.InputProofWithSync,
      inject: [uc.InputProof, SYNC_SERVICE, SyncInstances],
      useFactory: (
        inputProof: uc.InputProof,
        syncService: SyncService,
        syncInstances: SyncInstances,
      ) => new uc.InputProofWithSync(inputProof, syncService, syncInstances),
    },
    {
      provide: uc.INPUT_PROOF,
      inject: [uc.InputProofWithSync, API_KEY_ALLOWS_REQUEST],
      useFactory: (
        inputProof: uc.InputProofWithSync,
        apiKeyAllowsRequest: IApiKeyAllowsRequest,
      ) => new uc.InputProofWithApiKey(inputProof, apiKeyAllowsRequest),
    },
    {
      provide: uc.PRIVATE_DECRYPT,
      useClass: uc.PrivateDecrypt,
    },
    uc.GetKeyUrl,
  ],
})
export class RestModule {}
