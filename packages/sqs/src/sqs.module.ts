import { DynamicModule, Global, Module } from '@nestjs/common';
import { SqsOptions } from './sqs.types';
import { DiscoveryModule, DiscoveryService } from '@golevelup/nestjs-discovery';
import { SQS_OPTIONS } from './sqs.constants';
import { SqsService } from './sqs.service';

@Global()
@Module({})
export class SqsModule {
  public static register(options: SqsOptions): DynamicModule {
    return {
      global: true,
      module: SqsModule,
      imports: [DiscoveryModule],
      providers: [
        {
          provide: SQS_OPTIONS,
          useValue: options,
        },
        {
          provide: SqsService,
          inject: [DiscoveryService],
          useFactory: (discover: DiscoveryService) =>
            new SqsService(options, discover),
        },
      ],
    };
  }
}
