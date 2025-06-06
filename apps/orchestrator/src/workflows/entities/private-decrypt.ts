import { back, relayer } from 'messages'
import { Actor, createActor, setup } from 'xstate'

export const EVENT_TYPES = [
  'back:httpz:private-decrypt:requested',
  'relayer:private-decryption:operation-response',
] as const

export type PrivateDecryptEvents = Extract<
  back.BackEvent | relayer.RelayerEvent,
  { type: (typeof EVENT_TYPES)[number] }
>

export function isPrivateDecrypt(
  event: back.BackEvent | relayer.RelayerEvent,
): event is PrivateDecryptEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}

type PrivateDecryptMachine = ReturnType<typeof factory>

function factory({
  notifyMessage,
}: {
  notifyMessage: (message: back.BackEvent | relayer.RelayerEvent) => void
}) {
  return setup({
    types: {
      events: {} as PrivateDecryptEvents,
    },
  }).createMachine({
    id: 'inputProof',
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'back:httpz:private-decrypt:requested': {
            actions: [
              ({ event: { payload } }) =>
                notifyMessage(
                  relayer.privateDecryptionOperationRequest(payload),
                ),
            ],
          },
          'relayer:private-decryption:operation-response': {
            actions: [
              ({ event: { payload } }) =>
                notifyMessage(
                  back.httpzPrivateDecryptCompleted(payload, {
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

function chainIdToNumber(chainId: string | number): number {
  if (typeof chainId === 'string') {
    return parseInt(chainId, chainId.toLowerCase().startsWith('0x') ? 16 : 10)
  }
  return chainId
}

export class PrivateDecrypt {
  #actor: Actor<PrivateDecryptMachine>

  constructor() {
    this.#actor = createActor(
      factory({ notifyMessage: this.notifyMessage }),
    ).start()
  }

  private messages: (back.BackEvent | relayer.RelayerEvent)[] = []
  private notifyMessage = (message: back.BackEvent | relayer.RelayerEvent) => {
    this.messages.push(message)
  }

  send(event: PrivateDecryptEvents): (back.BackEvent | relayer.RelayerEvent)[] {
    this.messages = []
    this.#actor.send(event)
    return this.messages
  }
}
