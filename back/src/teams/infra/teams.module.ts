import { Module } from '@nestjs/common'
import { AuthModule } from 'src/auth/infra/auth.module'
import { DatabaseModule } from 'src/infra/database/database.module'

@Module({
  imports: [DatabaseModule, AuthModule],
  providers: [],
})
export class TeamsModule {}
