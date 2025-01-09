import { Module } from '@nestjs/common'
import { DatabaseService } from './database.service.js'

@Module({
  providers: [DatabaseService],
  exports: [DatabaseService],
})
export class DatabaseModule {}
