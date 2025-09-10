import { Module } from '@nestjs/common'
import { ConfigModule, ConfigService } from '@nestjs/config'
import { LoggerModule } from 'nestjs-pino'
import { randomUUID } from 'crypto'

import configuration from '#config/configuration.js'
import { WebhooksModule } from '#modules/webhooks/app/webhooks.module.js'

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
      useFactory: (config: ConfigService) => {
        return {
          pinoHttp: {
            level: config.get('common.logLevel', 'info'),
            customProps: () => ({ service: 'back' }),
            genReqId: request =>
              request.headers['x-correlation-id'] || randomUUID(),
            transport: {
              target: 'pino-pretty',
              options: {
                singleLine: true,
              },
            },
          },
        }
      },
    }),
    WebhooksModule,
  ],
})
export class AppModule {}
