import { Module } from '@nestjs/common';
import { SqsModule } from 'sqs';
import { AppController } from './app.controller';
import { AppService } from './app.service';

@Module({
  imports: [
    SqsModule.register({
      consumers: [],
    }),
  ],
  controllers: [AppController],
  providers: [AppService],
})
export class AppModule {}
