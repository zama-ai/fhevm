import {
  DynamicModule,
  Global,
  Module,
  ModuleMetadata,
  Provider,
  Type,
} from '@nestjs/common'
import {
  FeatureFlagHandler,
  FEATURE_FLAGS_SERVICE,
} from './services/feature-flags.service.js'

type FeatureFlagModuleOptions = {
  global?: boolean
  handlers: FeatureFlagHandler[]
}

type FeatureFlagOptionsFactory = {
  createFeatureFlagModuleOptions: () =>
    | Promise<FeatureFlagModuleOptions>
    | FeatureFlagModuleOptions
}

type FeatureFlagModuleAsyncOptions = Pick<ModuleMetadata, 'imports'> & {
  global?: boolean
  inject?: any[]
  extraProviders?: Provider[]
} & (
    | {
        useExisting: Type<FeatureFlagOptionsFactory>
        useClass?: never
        useFactory?: never
      }
    | {
        useExisting?: never
        useClass: Type<FeatureFlagOptionsFactory>
        useFactory?: never
      }
    | {
        useExisting?: never
        useClass?: never
        useFactory: (
          ...args: any[]
        ) => Promise<FeatureFlagModuleOptions> | FeatureFlagModuleOptions
      }
  )

const FEATURE_FLAG_MODULE_OPTIONS = 'FEATURE_FLAG_MODULE_OPTIONS'

@Global()
@Module({})
export class FeatureFlagModule {
  static register(options: FeatureFlagModuleOptions): DynamicModule {
    return {
      module: FeatureFlagModule,
      global: options.global,
      providers: [
        {
          provide: FEATURE_FLAG_MODULE_OPTIONS,
          useValue: options || {},
        },
        this.createServiceProvider(),
      ],
      exports: [FEATURE_FLAGS_SERVICE],
    }
  }

  static registerAsync(options: FeatureFlagModuleAsyncOptions): DynamicModule {
    return {
      module: FeatureFlagModule,
      global: options.global,
      imports: options.imports,
      providers: [
        ...this.createAsyncProviders(options),
        ...(options.extraProviders || []),
      ],
      exports: [FEATURE_FLAGS_SERVICE],
    }
  }

  private static createAsyncProviders(
    options: FeatureFlagModuleAsyncOptions,
  ): Provider[] {
    if (options.useExisting || options.useFactory) {
      return [
        this.createAsyncOptionsProvider(options),
        this.createServiceProvider(),
      ]
    }
    return [
      this.createAsyncOptionsProvider(options),
      {
        provide: options.useClass,
        useClass: options.useClass,
      },
      this.createServiceProvider(),
    ]
  }

  private static createAsyncOptionsProvider(
    options: FeatureFlagModuleAsyncOptions,
  ): Provider {
    if (options.useFactory) {
      return {
        provide: FEATURE_FLAG_MODULE_OPTIONS,
        useFactory: options.useFactory,
        inject: options.inject || [],
      }
    }
    return {
      provide: FEATURE_FLAG_MODULE_OPTIONS,
      useFactory: async (optionsFactory: FeatureFlagOptionsFactory) =>
        optionsFactory.createFeatureFlagModuleOptions(),
      inject: [options.useExisting || options.useClass],
    }
  }

  private static createServiceProvider(): Provider {
    return {
      provide: FEATURE_FLAGS_SERVICE,
      useFactory: (options: FeatureFlagModuleOptions) => {
        options.handlers.forEach((handler, i, handlers) => {
          if (i < handlers.length - 1) {
            handler.setNext(handlers[i + 1])
          }
        })
        return options.handlers[0]
      },
      inject: [FEATURE_FLAG_MODULE_OPTIONS],
    }
  }
}
