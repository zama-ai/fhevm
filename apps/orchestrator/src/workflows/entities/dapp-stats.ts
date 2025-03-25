import { back, web3 } from 'messages'
import { Actor, createActor, setup } from 'xstate'

export const EVENT_TYPES = [
  'back:dapp:stats-requested',
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
  notifyMessage: (message: back.BackEvent | web3.Web3Event) => void
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
            actions: [
              ({ event: { payload, meta } }) =>
                notifyMessage(web3.fheRequested(payload, meta)),
            ],
          },

          'web3:fhe-event:detected': {
            actions: ({
              event: {
                payload: { events, ...payload },
                meta,
              },
            }) =>
              notifyMessage(
                back.dappStatsAvailable(
                  {
                    ...payload,
                    events: events.map(({ id, ...event }) => ({
                      ...event,
                      externalRef: id,
                    })),
                  },
                  meta,
                ),
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

  private messages: (back.BackEvent | web3.Web3Event)[] = []
  private notifyMessage = (message: back.BackEvent | web3.Web3Event) => {
    this.messages.push(message)
  }

  send(event: DAppStatsEvents): (back.BackEvent | web3.Web3Event)[] {
    this.messages = []
    this.#actor.send(event)
    return this.messages
  }
}
