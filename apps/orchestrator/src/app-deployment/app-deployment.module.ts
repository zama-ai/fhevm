import { Module } from '@nestjs/common';
import { ProcessEventUseCase } from './use-cases/process-event.use-case';

@Module({
  providers: [ProcessEventUseCase],
  exports: [ProcessEventUseCase],
})
export class AppDeploymentModule {}
