import { Module } from '@nestjs/common'
import { WebhooksController } from './webhooks.controller.js'
import { WebhookService } from './webhooks.service.js'
import { DeveloperPortalService } from './developer-portal.service.js'
import { HttpModule } from '@nestjs/axios'
import { ConfigService } from '@nestjs/config'

@Module({
  imports: [
    HttpModule.registerAsync({
      inject: [ConfigService],
      useFactory: (config: ConfigService) => ({
        baseURL: config.getOrThrow('tyk.endpoint'),
        headers: {
          Authorization: config.getOrThrow('tyk.apiKey'),
        },
      }),
    }),
  ],
  controllers: [WebhooksController],
  providers: [WebhookService, DeveloperPortalService],
})
export class WebhooksModule {}
