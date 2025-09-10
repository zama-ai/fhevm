import { ZodValidationPipe } from '#pipes/zod-validation.pipe.js'
import {
  BadRequestException,
  Body,
  Controller,
  HttpCode,
  HttpStatus,
  Post,
} from '@nestjs/common'
import { type WebhookPayload, WebhookPayloadSchema } from './webhooks.types.js'
import { WebhookService } from './webhooks.service.js'

@Controller('webhooks')
export class WebhooksController {
  constructor(private readonly service: WebhookService) {}

  @Post('/')
  @HttpCode(HttpStatus.ACCEPTED)
  async webhook(
    @Body(new ZodValidationPipe(WebhookPayloadSchema))
    payload: WebhookPayload,
  ) {
    switch (payload.Event) {
      case 'UserRegistered':
        await this.service.handleUserRegistered(payload.Message)
        break

      case 'ApplicationRegistered':
        await this.service.handleApplicationRegistered(payload.Message)
        break

      default:
        throw new BadRequestException('Unknown event')
    }
  }
}
