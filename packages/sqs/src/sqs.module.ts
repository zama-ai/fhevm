import { DynamicModule, Global, Module, Provider, Type } from '@nestjs/common';
import type {
  SqsModuleAsyncOptions,
  SqsModuleOptionsFactory,
  SqsOptions,
} from './sqs.types.js';
import { DiscoveryModule, DiscoveryService } from '@golevelup/nestjs-discovery';
import { SQS_OPTIONS } from './sqs.constants.js';
import { SqsService } from './sqs.service.js';

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

  public static registerAsync(options: SqsModuleAsyncOptions): DynamicModule {
    const asyncProviders = this.createAsyncProviders(options);
    const sqsProvider: Provider = {
      provide: SqsService,
      useFactory: (options: SqsOptions, discover: DiscoveryService) =>
        new SqsService(options, discover),
      inject: [SQS_OPTIONS, DiscoveryService],
    };

    return {
      global: true,
      module: SqsModule,
      imports: [DiscoveryModule, ...(options.imports ?? [])],
      providers: [...asyncProviders, sqsProvider],
      exports: [sqsProvider],
    };
  }

  private static createAsyncProviders(
    options: SqsModuleAsyncOptions,
  ): Provider[] {
    if (options.useExisting || options.useFactory) {
      return [this.createAsyncOptionsProvider(options)];
    }
    const useClass = options.useClass as Type<SqsModuleOptionsFactory>;
    return [
      this.createAsyncOptionsProvider(options),
      {
        provide: useClass,
        useClass,
      },
    ];
  }

  private static createAsyncOptionsProvider(
    options: SqsModuleAsyncOptions,
  ): Provider {
    if (options.useFactory) {
      return {
        provide: SQS_OPTIONS,
        useFactory: options.useFactory,
        inject: options.inject || [],
      };
    }

    const inject = [
      (options.useClass ||
        options.useExisting) as Type<SqsModuleOptionsFactory>,
    ];
    return {
      provide: SQS_OPTIONS,
      useFactory: async (optionsFactory: SqsModuleOptionsFactory) =>
        await optionsFactory.createOptions(),
      inject,
    };
  }
}
