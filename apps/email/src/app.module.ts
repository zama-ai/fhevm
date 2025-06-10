import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import configuration from './config/configuration.js'
import { InfraModule } from '#workflows/infra/infra.module.js'
import { LoggerModule } from 'nestjs-pino'
import { randomUUID } from 'crypto'

// Note: I need to override the default behavior of ConfigModule in the tests,
// and, as we use a dynamic module, we need to store the current instance to
// override it in the tests.
export const configModule = ConfigModule.forRoot({
  isGlobal: true,
  load: [configuration],
})

@Module({
  imports: [
    configModule,
    LoggerModule.forRootAsync({
      imports: [configModule],
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        pinoHttp: {
          level: config.get('common.logLevel', 'info'),
          customProps: () => ({ service: 'email' }),
          genReqId: request =>
            request.headers['x-correlation-id'] || randomUUID(),
          transport:
            config.get('common.nodeEnv') === 'development'
              ? {
                  target: 'pino-pretty',
                  options: {
                    singleLine: true,
                  },
                }
              : undefined,
        },
      }),
    }),
    InfraModule,
  ],
})
export class AppModule {}
