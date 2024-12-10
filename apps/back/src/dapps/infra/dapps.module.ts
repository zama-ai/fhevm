import { Module } from '@nestjs/common'
import { DappsResolver } from './dapps.resolver'
import { DatabaseModule } from '@/infra/database/database.module'
import { CreateDapp } from '@/dapps/use-cases/create-dapp.use-case'
import { UpdateDapp } from '../use-cases/update-dapp.use-case'

@Module({
  imports: [DatabaseModule],
  providers: [DappsResolver, CreateDapp, UpdateDapp],
})
export class DappsModule {}
