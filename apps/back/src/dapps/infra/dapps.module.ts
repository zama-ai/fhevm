import { Module } from '@nestjs/common'
import { DappsResolver } from './dapps.resolver'
import { DatabaseModule } from '@/infra/database/database.module'
import { CreateDapp } from '@/dapps/use-cases/create-dapp.use-case'
import { UpdateDapp } from '@/dapps/use-cases/update-dapp.use-case'
import { GetTeamById } from '@/users/use-cases/get-team-by-id.use-case'
import { GetDappById } from '../use-cases/get-dapp-by-id.use-case'

@Module({
  imports: [DatabaseModule],
  providers: [DappsResolver, CreateDapp, UpdateDapp, GetTeamById, GetDappById],
})
export class DappsModule {}
