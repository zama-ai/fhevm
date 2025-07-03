import { back, email } from 'messages'
import { type Actor, createActor, setup } from 'xstate'

export const EVENT_TYPES = [
  'back:user:created',
  'back:password-reset:completed',
  'back:password-reset:requested',
] as const

export type AuthEvents = Extract<
  back.BackEvent | email.EmailEvent,
  { type: (typeof EVENT_TYPES)[number] }
>

export function isAuthEvent(
  event: back.BackEvent | email.EmailEvent,
): event is AuthEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}

type AuthMachine = ReturnType<typeof factory>

function factory({
  notifyMessage,
}: {
  notifyMessage: (message: back.BackEvent | email.EmailEvent) => void
}) {
  return setup({
    types: {
      events: {} as AuthEvents,
    },
  }).createMachine({
    id: 'auth',
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'back:user:created': {
            actions: [
              ({
                event: {
                  payload: { requestId, email: userEmail, token },
                  meta,
                },
              }) =>
                notifyMessage(
                  email.userCreated(
                    { requestId, email: userEmail, token },
                    meta,
                  ),
                ),
            ],
          },
          'back:password-reset:requested': {
            actions: [
              ({ event: { payload, meta } }) =>
                notifyMessage(email.passwordResetRequested(payload, meta)),
            ],
          },

          'back:password-reset:completed': {
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

export class Auth {
  #actor: Actor<AuthMachine>
  constructor() {
    this.#actor = createActor(factory({ notifyMessage: this.notifyMessage }))
    this.#actor.start()
  }
  private messages: (back.BackEvent | email.EmailEvent)[] = []
  private notifyMessage = (message: back.BackEvent | email.EmailEvent) => {
    this.messages.push(message)
  }
  send(event: AuthEvents): (back.BackEvent | email.EmailEvent)[] {
    this.messages = []
    this.#actor.send(event)
    return this.messages
  }
}
