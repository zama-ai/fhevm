import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import { randomUUID } from 'crypto'
import { LoggerModule } from 'nestjs-pino'
import { InfraModule } from './infra/infra.module.js'
import { MS_NAME } from '#constants.js'
import config from '#config/index.js'

// Note: I need to override the default behavior of ConfigModule in the tests,
// and, as we use a dynamic module, we need to store the current instance to
// override it in the tests.
export const configModule = ConfigModule.forRoot({
  isGlobal: true,
  load: config,
})

@Module({
  imports: [
    configModule,
    LoggerModule.forRootAsync({
      imports: [configModule],
      inject: [ConfigService],
      useFactory: (config: ConfigService) => {
        return {
          pinoHttp: {
            level: config.get('common.logLevel'),
            customProps: () => ({ service: MS_NAME }),
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
        }
      },
    }),
    InfraModule,
  ],
})
export class AppModule {}
