import { Email } from '#domain/email.js'
import {
  EMAIL_SERVICE,
  type EmailService,
} from '#workflows/use-cases/adapters/email.service.js'
import {
  isPasswordResetRequested,
  PasswordResetRequested,
  PasswordResetRequestedProducer,
} from '#workflows/use-cases/gateways/password-reset-requested.producer.js'
import {
  isUserCreated,
  UserCreated,
} from '#workflows/use-cases/gateways/user-created.producer.js'
import { Inject, Injectable, Logger } from '@nestjs/common'
import { ConfigService } from '@nestjs/config'
import { email } from 'messages'
import { Task, AppError, unknownError } from 'utils'

@Injectable()
export class SESProducer implements PasswordResetRequestedProducer {
  private readonly logger = new Logger(SESProducer.name)

  private readonly frontendUrl: string
  constructor(
    @Inject(EMAIL_SERVICE) private readonly emailService: EmailService,
    private readonly configService: ConfigService,
  ) {
    const url = this.configService.getOrThrow<string>('ses.frontendUrl')
    this.frontendUrl = url.endsWith('/') ? url : url + '/'
  }

  produce(event: PasswordResetRequested): Task<void, AppError>
  produce(event: UserCreated): Task<void, AppError>
  produce(event: email.EmailEvent): Task<void, AppError> {
    this.logger.debug(`sending email to ${event.payload.email}`)

    let email: Email | undefined

    if (isPasswordResetRequested(event)) {
      email = {
        to: event.payload.email,
        from: this.configService.get('ses.fromEmail') || '',
        subject: 'Reset your password',
        data: {
          context: {
            token: event.payload.token,
            frontendUrl: this.frontendUrl,
          },
          template: 'send-reset-token.ejs',
        },
      }
    }
    if (isUserCreated(event)) {
      email = {
        to: event.payload.email,
        from: this.configService.get('ses.fromEmail') || '',
        subject: 'Confirm your email',
        data: {
          context: {
            token: event.payload.token,
            frontendUrl: this.frontendUrl,
          },
          template: 'send-confirmation-token.ejs',
        },
      }
    }

    return email
      ? this.emailService.sendEmail(email)
      : Task.reject(unknownError('unknown event'))
  }
}
