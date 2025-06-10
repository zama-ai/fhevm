import {
  EMAIL_SERVICE,
  type EmailService,
} from '#workflows/use-cases/adapters/email.service.js'
import {
  PasswordResetRequested,
  PasswordResetRequestedProducer,
} from '#workflows/use-cases/gateways/password-reset-requested.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { Task, AppError, unknownError } from 'utils'

@Injectable()
export class SESProducer implements PasswordResetRequestedProducer {
  private readonly logger = new Logger(SESProducer.name)
  constructor(
    @Inject(EMAIL_SERVICE) private readonly emailService: EmailService,
    private readonly configService: ConfigService,
  ) {}

  private get frontendUrl(): string {
    const url = this.configService.getOrThrow<string>('ses.frontendUrl')
    return url.endsWith('/') ? url : url + '/'
  }

  produce(event: PasswordResetRequested): Task<void, AppError> {
    this.logger.debug(`sending email to ${event.payload.email}`)
    let frontendUrl: string
    try {
      frontendUrl = this.frontendUrl
    } catch (err) {
      this.logger.warn(`failed to get frontendUrl: ${err}`)
      return Task.reject<void, AppError>(
        unknownError(`failed to get frontendUrl: ${err}`),
      )
    }
    return this.emailService.sendEmail({
      to: event.payload.email,
      from: this.configService.get('ses.fromEmail') || '',
      subject: 'Reset your password',
      data: {
        context: {
          token: event.payload.token,
          frontendUrl,
        },
        template: 'send-reset-token.ejs',
      },
    })
  }
}
