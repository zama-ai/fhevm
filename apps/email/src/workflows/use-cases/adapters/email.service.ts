import { Email } from '#domain/email.js'
import { AppError, Task } from 'utils'

export const EMAIL_SERVICE = 'EMAIL_SERVICE'

export interface EmailService {
  sendEmail(email: Email): Task<void, AppError>
}
