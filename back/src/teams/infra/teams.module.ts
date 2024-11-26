import { Module } from '@nestjs/common'
import { AuthModule } from '@/auth/infra/auth.module'
import { DatabaseModule } from '@/infra/database/database.module'

@Module({
  imports: [DatabaseModule, AuthModule],
  providers: [],
})
export class TeamsModule {}
