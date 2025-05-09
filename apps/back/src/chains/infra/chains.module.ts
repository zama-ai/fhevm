import { DatabaseModule } from '#infra/database/database.module.js'
import { Module } from '@nestjs/common'
import { ChainsResolver } from './graphql/chains.resolver.js'
import { CHAINS_REPOSITORY } from '#chains/domain/repositories/chains.repository.js'
import { PrismaChainsRepository } from './db/prisma-chains.repository.js'
import * as uc from '#chains/use-cases/index.js'
@Module({
  imports: [DatabaseModule],
  providers: [
    {
      provide: CHAINS_REPOSITORY,
      useClass: PrismaChainsRepository,
    },
    uc.GetAllChains,
    uc.GetChainById,
    ChainsResolver,
  ],
  exports: [uc.GetChainById],
})
export class ChainsModule {}
