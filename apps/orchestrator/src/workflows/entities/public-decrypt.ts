import { back, relayer } from 'messages'
import { Actor, createActor, setup } from 'xstate'

export const EVENT_TYPES = [
  'back:httpz:public-decrypt:requested',
  'relayer:public-decryption:operation-response',
] as const

export type PublicDecryptEvents = Extract<
  back.BackEvent | relayer.RelayerEvent,
  { type: (typeof EVENT_TYPES)[number] }
>

export function isPublicDecrypt(
  event: back.BackEvent | relayer.RelayerEvent,
): event is PublicDecryptEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}

type PublicDecryptMachine = ReturnType<typeof factory>

function factory({
  notifyMessage,
}: {
  notifyMessage: (message: back.BackEvent | relayer.RelayerEvent) => void
}) {
  return setup({
    types: {
      events: {} as PublicDecryptEvents,
    },
  }).createMachine({
    id: 'inputProof',
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'back:httpz:public-decrypt:requested': {
            actions: [
              ({ event: { payload } }) =>
                notifyMessage(
                  relayer.publicDecryptionOperationRequest(payload),
                ),
            ],
          },
          'relayer:public-decryption:operation-response': {
            actions: [
              ({ event: { payload } }) =>
                notifyMessage(
                  back.httpzPublicDecryptCompleted(payload, {
                    correlationId: payload.requestId,
                  }),
                ),
            ],
          },
        },
      },
    },
  })
}

export class PublicDecrypt {
  #actor: Actor<PublicDecryptMachine>

  constructor() {
    this.#actor = createActor(
      factory({ notifyMessage: this.notifyMessage }),
    ).start()
  }

  private messages: (back.BackEvent | relayer.RelayerEvent)[] = []
  private notifyMessage = (message: back.BackEvent | relayer.RelayerEvent) => {
    this.messages.push(message)
  }

  send(event: PublicDecryptEvents): (back.BackEvent | relayer.RelayerEvent)[] {
    this.messages = []
    this.#actor.send(event)
    return this.messages
  }
}
