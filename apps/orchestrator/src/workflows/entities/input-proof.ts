import { back, relayer } from 'messages'
import { type Actor, createActor, setup } from 'xstate'

export const EVENT_TYPES = [
  'back:httpz:input-proof:requested',
  'relayer:input-registration:input-registration-response',
] as const

export type InputProofEvents = Extract<
  back.BackEvent | relayer.RelayerEvent,
  { type: (typeof EVENT_TYPES)[number] }
>

export function isInputProofEvent(
  event: back.BackEvent | relayer.RelayerEvent,
): event is InputProofEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}

type InputProofMachine = ReturnType<typeof factory>

function factory({
  notifyMessage,
}: {
  notifyMessage: (message: back.BackEvent | relayer.RelayerEvent) => void
}) {
  return setup({
    types: {
      events: {} as InputProofEvents,
    },
  }).createMachine({
    id: 'inputProof',
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'back:httpz:input-proof:requested': {
            actions: [
              ({ event: { payload, meta } }) =>
                notifyMessage(
                  relayer.inputRegistrationRequest(
                    {
                      ...payload,
                      // NOTE: Relayer expects a number
                      contractChainId: chainIdToNumber(payload.contractChainId),
                    },
                    meta,
                  ),
                ),
            ],
          },
          'relayer:input-registration:input-registration-response': {
            actions: [
              ({ event: { payload, meta } }) =>
                notifyMessage(back.httpzInputProofCompleted(payload, meta)),
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

export class InputProof {
  #actor: Actor<InputProofMachine>

  constructor() {
    this.#actor = createActor(
      factory({ notifyMessage: this.notifyMessage }),
    ).start()
  }

  private messages: (back.BackEvent | relayer.RelayerEvent)[] = []
  private notifyMessage = (message: back.BackEvent | relayer.RelayerEvent) => {
    this.messages.push(message)
  }

  send(event: InputProofEvents): (back.BackEvent | relayer.RelayerEvent)[] {
    this.messages = []
    this.#actor.send(event)
    return this.messages
  }
}
