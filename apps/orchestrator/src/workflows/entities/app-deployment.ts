import { Actor, assign, createActor, setup, Snapshot } from 'xstate'
import { back, web3 } from 'messages'

export interface AppDeploymentProps {
  requestId: string
  // dAppId: string
  // chainId: string
  // address: string
}

export const EVENT_TYPES = [
  'back:dapp:validation:requested',
  'back:dapp:validation:confirmed',
  'back:dapp:validation:failed',
  'web3:contract:validation:requested',
  'web3:contract:validation:success',
  'web3:contract:validation:failure',
] as const

export type AppDeploymentEvents = Extract<
  back.BackEvent | web3.Web3Event,
  {
    type: (typeof EVENT_TYPES)[number]
  }
>

export function isAppDeploymentEvent(
  event: back.BackEvent | web3.Web3Event,
): event is AppDeploymentEvents {
  return (EVENT_TYPES as readonly string[]).includes(event.type)
}

type Context = {
  requestId: string
  dAppId: string
  chainId: string
  address: string
  messages: (back.BackEvent | web3.Web3Event)[]
}

type AppDeploymentMachine = ReturnType<typeof factory>

function factory({
  requestId,
  // dAppId,
  // chainId,
  // address,
  notifyMessage,
}: Pick<Context, 'requestId'> & {
  notifyMessage: (message: AppDeploymentEvents) => void
}) {
  return setup({
    types: {
      context: {} as Context,
      events: {} as AppDeploymentEvents,
    },
    guards: {
      isValid: ({ context, event }) =>
        context.requestId === event.payload.requestId,
    },
  }).createMachine({
    /** @xstate-layout N4IgpgJg5mDOIC5QEMAOqAiZUBsD2AngLZgB2ALgHQCSEOYAxGqgLQTb7FlUBOYAjgFc45SAG0ADAF1EoVHlgBLcorylZIAB6IAjACYAbJQkmTAZj2WArGasGAHABoQBRPZ2UALAE5f3wwDsBhIB-gEAvuHOzFi4hCQUlBiKsADGeABuYDyKpFBM6Gwc8dyUaWwp6Vl8EJIySCDySipqGtoIVt4SlAEBEvZWEp4SVno6Vk4ubvZe9gbBVjreATpDBnqR0eixnAlUAMJqAGaKPES5+cxFcVyJ5emkJ2fi0hpNyqrqDe2rVgGUYz0yx0fXWXUmrgQ9hmnjmCyWKzWGyiIBixVuVAASmAoClRDk8gVWOwbnsyqkWHxcbB8S96nIFB9Wt9dD5ut4dJ55rYzPYTHoAs5IfY9MZTBIDN4fJ4BYZIijSHh2PAGmjSdw3oyWl9QO0WAYhYh9WLTGZPEsDGZvLzPJtUdt0WTaPRNc1Pm1EDLDQhVp4eoY7PNPFaDJ5Rna1btSsk0plshdXUydVpPeNKGYQSLYQK+RJvAapj6hv6DIHQyGw2YIw71YlDo9Tuc8ontR6EF12Tp7LZ8zorYNPN7fSWy8H82HkVtMI7StjqfiEw13q2WT6zd0AubBr1N6E9IPC8OAgH5uXx+GUZGSokAIKpFRZFvu1eZmZmM29ANWGUiocSMxeEEBg6CBiLvhm8rhEAA */
    id: 'appDeployment',
    context: { requestId, dAppId: '', chainId: '', address: '', messages: [] },
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'back:dapp:validation:requested': {
            target: 'Confirming',
            actions: [
              assign(
                ({
                  event: {
                    payload: { dAppId, chainId, address },
                    meta,
                  },
                }) => ({
                  dAppId,
                  address,
                  chainId,
                  meta,
                }),
              ),
              ({
                event: {
                  payload: { requestId, chainId, address },
                  meta,
                },
              }) =>
                notifyMessage(
                  web3.contractValidationRequested(
                    { requestId, chainId, address },
                    meta,
                  ),
                ),
            ],
          },
        },
      },
      Confirming: {
        on: {
          'web3:contract:validation:success': {
            guard: 'isValid',
            target: 'Completed',
            actions: [
              ({ context: { requestId, dAppId }, event: { meta } }) =>
                notifyMessage(
                  back.dappValidationConfirmed({ requestId, dAppId }, meta),
                ),
            ],
          },
          'web3:contract:validation:failure': {
            guard: 'isValid',
            target: 'Completed',
            actions: [
              ({
                context: { requestId, dAppId },
                event: {
                  payload: { reason },
                  meta,
                },
              }) =>
                notifyMessage(
                  back.dappValidationFailed(
                    {
                      requestId,
                      dAppId,
                      reason: reason || 'Failed to check smart contract',
                    },
                    meta,
                  ),
                ),
            ],
          },
        },
      },
      Completed: {
        type: 'final',
      },
    },
  })
}
export class AppDeployment {
  #actor: Actor<AppDeploymentMachine>

  constructor({ requestId }: AppDeploymentProps, snapshot?: string) {
    this.#actor = createActor(
      factory({
        requestId,
        notifyMessage: this.notifyMessage,
      }),
      {
        snapshot: snapshot
          ? (JSON.parse(snapshot) as Snapshot<unknown>)
          : undefined,
      },
    )
    this.#actor.start()
  }

  private messages: AppDeploymentEvents[] = []
  private notifyMessage = (message: AppDeploymentEvents) => {
    this.messages.push(message)
  }

  send(event: AppDeploymentEvents): AppDeploymentEvents[] {
    this.messages = []
    this.#actor.send(event)
    return this.messages
  }

  get status() {
    return this.#actor.getSnapshot().value
  }

  get requestId() {
    return this.#actor.getSnapshot().context.requestId
  }

  get snapshot() {
    return JSON.stringify(this.#actor.getPersistedSnapshot())
  }

  get isComplete() {
    return this.#actor.getSnapshot().status === 'done'
  }
}
