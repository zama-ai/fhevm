import { email } from 'messages'
import { AppError, Task } from 'utils'

export type UserCreated = Extract<
  email.EmailEvent,
  {
    type: 'email:user:created'
  }
>

export interface UserCreatedProducer {
  produce: (event: UserCreated) => Task<void, AppError>
}

export const USER_CREATED_PRODUCER = 'USER_CREATED_PRODUCER'

export function isUserCreated(event: email.EmailEvent): event is UserCreated {
  return event.type === 'email:user:created'
}
