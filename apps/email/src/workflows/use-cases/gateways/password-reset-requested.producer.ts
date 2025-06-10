import { email } from 'messages'
import { AppError, Task } from 'utils'

export type PasswordResetRequested = Extract<
  email.EmailEvent,
  {
    type: 'email:password-reset:requested'
  }
>

export interface PasswordResetRequestedProducer {
  produce(event: PasswordResetRequested): Task<void, AppError>
}

export const PASSWORD_RESET_REQUESTED_PRODUCER =
  'PASSWORD_RESET_REQUESTED_PRODUCER'
