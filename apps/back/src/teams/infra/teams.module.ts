import { Module } from '@nestjs/common'
import { AuthModule } from '#auth/infra/auth.module.js'
import { DatabaseModule } from '#infra/database/database.module.js'

@Module({
  imports: [DatabaseModule, AuthModule],
  providers: [],
})
export class TeamsModule {}
