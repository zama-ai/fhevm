import { back, email } from 'messages'
import { type Actor, createActor, setup } from 'xstate'

export const EVENT_TYPES = [
  'back:user:password-reset:completed',
  'back:user:password-reset:requested',
] as const

export type PasswordResetEvents = Extract<
  back.BackEvent | email.EmailEvent,
  { type: (typeof EVENT_TYPES)[number] }
>

export function isPasswordResetEvent(
  event: back.BackEvent | email.EmailEvent,
): event is PasswordResetEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}

type PasswordResetMachine = ReturnType<typeof factory>

function factory({
  notifyMessage,
}: {
  notifyMessage: (message: back.BackEvent | email.EmailEvent) => void
}) {
  return setup({
    types: {
      events: {} as PasswordResetEvents,
    },
  }).createMachine({
    id: 'passwordReset',
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'back:user:password-reset:requested': {
            actions: [
              ({ event: { payload, meta } }) =>
                notifyMessage(email.passwordResetRequested(payload, meta)),
            ],
          },

          'back:user:password-reset:completed': {
            actions: [
              ({ event: { payload, meta } }) =>
                notifyMessage(email.passwordResetCompleted(payload, meta)),
            ],
          },
        },
      },
    },
  })
}

export class PasswordReset {
  #actor: Actor<PasswordResetMachine>
  constructor() {
    this.#actor = createActor(factory({ notifyMessage: this.notifyMessage }))
    this.#actor.start()
  }
  private messages: (back.BackEvent | email.EmailEvent)[] = []
  private notifyMessage = (message: back.BackEvent | email.EmailEvent) => {
    this.messages.push(message)
  }
  send(event: PasswordResetEvents): (back.BackEvent | email.EmailEvent)[] {
    this.#actor.send(event)
    return this.messages
  }
}
