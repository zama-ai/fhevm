import { back, web3 } from 'messages'
import { Actor, createActor, setup } from 'xstate'

export const EVENT_TYPES = [
  'back:dapp:stats-requested',
  'back:dapp:stats-available',
  'web3:fhe-event:requested',
  'web3:fhe-event:detected',
] as const

type EventTypes = typeof EVENT_TYPES

export type DAppStatsEvents = Extract<
  back.BackEvent | web3.Web3Event,
  { type: EventTypes[number] }
>

export function isDAppStatsEvent(
  event: back.BackEvent | web3.Web3Event,
): event is DAppStatsEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}

type DAppStatsMachine = ReturnType<typeof factory>

function factory({
  notifyMessage,
}: {
  notifyMessage: (message: DAppStatsEvents) => void
}) {
  return setup({
    types: {
      events: {} as DAppStatsEvents,
    },
  }).createMachine({
    id: 'dAppStats',
    context: {},
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'back:dapp:stats-requested': {
            actions: ({ event: { payload, meta } }) =>
              notifyMessage(web3.fheRequested(payload, meta)),
          },

          'web3:fhe-event:detected': {
            actions: ({
              event: {
                payload: { id, ...payload },
                meta,
              },
            }) =>
              notifyMessage(
                back.dappStatsAvailable({ ...payload, externalRef: id }, meta),
              ),
          },
        },
      },
    },
  })
}

export class DAppStats {
  #actor: Actor<DAppStatsMachine>

  constructor() {
    this.#actor = createActor(
      factory({
        notifyMessage: this.notifyMessage,
      }),
    )
    this.#actor.start()
  }

  private messages: DAppStatsEvents[] = []
  private notifyMessage = (message: DAppStatsEvents) => {
    this.messages.push(message)
  }

  send(event: DAppStatsEvents): DAppStatsEvents[] {
    this.messages = []
    this.#actor.send(event)
    return this.messages
  }
}
