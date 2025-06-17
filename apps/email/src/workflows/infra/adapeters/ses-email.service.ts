import {
  SendEmailCommand,
  SendEmailCommandOutput,
  SESClient,
} from '@aws-sdk/client-ses'
import { EmailService } from '#workflows/use-cases/adapters/email.service.js'
import { ConfigService } from '@nestjs/config'
import { Inject, Injectable, Logger } from '@nestjs/common'
import {
  TEMPLATE_ADAPTER,
  type TemplateAdapter,
} from '#workflows/use-cases/adapters/template.adapter.js'
import { Email } from '#domain/email.js'
import { Task, AppError, unknownError } from 'utils'

@Injectable()
export class SESEmailService implements EmailService {
  private readonly logger = new Logger(SESEmailService.name)
  private _client: SESClient | undefined

  constructor(
    private readonly config: ConfigService,
    @Inject(TEMPLATE_ADAPTER) private readonly templateAdapter: TemplateAdapter,
  ) {}

  private get client(): SESClient {
    if (!this._client) {
      this.logger.verbose(`creating ses client`)
      this._client = new SESClient(
        // TODO: change to aws.useConfigCredentials when sqs is ready
        this.config.get<boolean>('ses.useConfigCredentials', false)
          ? {
              region: this.config.get('ses.region'),
              endpoint: this.config.get('ses.endpoint'),
              credentials: {
                accessKeyId: this.config.getOrThrow('ses.accessKeyId'),
                secretAccessKey: this.config.getOrThrow('ses.secretAccessKey'),
              },
            }
          : {},
      )
    }
    return this._client
  }

  sendEmail(email: Email): Task<void, AppError> {
    return this.templateAdapter.render(email).chain(html => {
      return Task.fromPromise<SendEmailCommandOutput, unknown>(async () => {
        this.logger.debug(
          `sending email "${email.subject}" to "${email.to}" from "${email.from}"`,
        )
        try {
          const output = await this.client.send(
            new SendEmailCommand({
              Destination: {
                ToAddresses: [email.to],
              },
              Message: {
                Body: {
                  Html: {
                    Charset: 'UTF-8',
                    Data: html,
                  },
                },
                Subject: {
                  Charset: 'UTF-8',
                  Data: email.subject,
                },
              },
              Source: email.from,
            }),
          )

          this.logger.debug(`status ${JSON.stringify(output)}`)
          return output
        } catch (err) {
          this.logger.warn(`failed to send email: ${err}`)
          throw err
        }
      })
        .chain<void>(output => {
          return output.$metadata.httpStatusCode === 200
            ? Task.of(void 0)
            : Task.reject(unknownError('Failed to send email'))
        })
        .mapError(err => {
          this.logger.warn(`failed to send email: ${err}`)
          return unknownError(`Failed to send email: ${err}`)
        })
    })
  }
}
