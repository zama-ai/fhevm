import { Module } from '@nestjs/common'
import { DatabaseModule } from 'src/infra/database/database.module'
import { AuthModule } from '../auth/auth.module'

@Module({
  imports: [DatabaseModule, AuthModule],
  providers: [],
})
export class UsersModule {}
