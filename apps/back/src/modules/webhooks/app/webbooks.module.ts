import { Module } from '@nestjs/common'
import { WebhooksController } from './webhooks.controller.js'
import { WebhookService } from './webhooks.service.js'
import { DeveloperPortalService } from './developer-portal.service.js'

@Module({
  controllers: [WebhooksController],
  providers: [WebhookService, DeveloperPortalService],
})
export class WebhooksModule {}
