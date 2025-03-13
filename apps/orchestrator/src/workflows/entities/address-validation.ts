import { back, web3 } from 'messages'
import { type Actor, createActor, setup } from 'xstate'

export const EVENT_TYPES = [
  'back:address:validation:requested',
  'web3:contract:validation:success',
  'web3:contract:validation:failure',
] as const

export type AddressValidationEvents = Extract<
  back.BackEvent | web3.Web3Event,
  { type: (typeof EVENT_TYPES)[number] }
>

export function isAddressValidationEvent(
  event: back.BackEvent | web3.Web3Event,
): event is AddressValidationEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}

type AddressValidationMachine = ReturnType<typeof factory>

function factory({
  notifyMessage,
}: {
  notifyMessage: (message: back.BackEvent | web3.Web3Event) => void
}) {
  return setup({
    types: {
      events: {} as AddressValidationEvents,
    },
  }).createMachine({
    id: 'addressValidation',
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'back:address:validation:requested': {
            actions: [
              ({ event: { payload, meta } }) =>
                notifyMessage(web3.contractValidationRequested(payload, meta)),
            ],
          },
          'web3:contract:validation:success': {
            actions: [
              ({ event: { payload, meta } }) =>
                notifyMessage(back.addressValidationConfirmed(payload, meta)),
            ],
          },
          'web3:contract:validation:failure': {
            actions: [
              ({ event }) => {
                console.log(`address-validation on ${event.type}`)
              },
              ({ event: { payload, meta } }) =>
                notifyMessage(
                  back.addressValidationFailed(
                    {
                      ...payload,
                      reason: payload.reason ?? 'no reason provided',
                    },
                    meta,
                  ),
                ),
            ],
          },
        },
      },
    },
  })
}

export class AddressValidation {
  #actor: Actor<AddressValidationMachine>

  constructor() {
    this.#actor = createActor(
      factory({
        notifyMessage: this.notifyMessage,
      }),
    ).start()
  }

  private messages: (back.BackEvent | web3.Web3Event)[] = []
  private notifyMessage = (message: back.BackEvent | web3.Web3Event) => {
    this.messages.push(message)
  }

  send(event: AddressValidationEvents): (back.BackEvent | web3.Web3Event)[] {
    this.messages = []
    this.#actor.send(event)
    return this.messages
  }
}
