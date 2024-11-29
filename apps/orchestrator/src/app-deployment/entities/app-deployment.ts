import { Actor, assign, createActor, setup, Snapshot } from 'xstate';
import { type AppDeploymentEvent, completed } from './app-deployment.events';
import { confirmSM, discoverSM, registerSM } from './app-deployment.commands';
import type { AppDeploymentMessage } from './app-deployment.messages';

type Context = {
  applicationId: string;
  address?: string;
  chainId?: string;
  messages: AppDeploymentMessage[];
};

type AppDeploymentMachine = ReturnType<typeof factory>;

function factory({ applicationId }: { applicationId: string }) {
  return setup({
    types: {
      context: {} as Context,
      events: {} as AppDeploymentEvent,
    },
  }).createMachine({
    /** @xstate-layout N4IgpgJg5mDOIC5QEMAOqAiZUBsD2AngLZgB2ALgHQCSEOYAxGqgLQTb7FlUBOYAjgFc45SAG0ADAF1EoVHlgBLcorylZIAB6IAjACYAbJQkmTAZj2WArGasGAHABoQBRPZ2UALAE5f3wwDsBhIB-gEAvuHOzFi4hCQUlBiKsADGeABuYDyKpFBM6Gwc8dyUaWwp6Vl8EJIySCDySipqGtoIVt4SlAEBEvZWEp4SVno6Vk4ubvZe9gbBVjreATpDBnqR0eixnAlUAMJqAGaKPES5+cxFcVyJ5emkJ2fi0hpNyqrqDe2rVgGUYz0yx0fXWXUmrgQ9hmnjmCyWKzWGyiIBixVuVAASmAoClRDk8gVWOwbnsyqkWHxcbB8S96nIFB9Wt9dD5ut4dJ55rYzPYTHoAs5IfY9MZTBIDN4fJ4BYZIijSHh2PAGmjSdw3oyWl9QO0WAYhYh9WLTGZPEsDGZvLzPJtUdt0WTaPRNc1Pm1EDLDQhVp4eoY7PNPFaDJ5Rna1btSsk0plshdXUydVpPeNKGYQSLYQK+RJvAapj6hv6DIHQyGw2YIw71YlDo9Tuc8ontR6EF12Tp7LZ8zorYNPN7fSWy8H82HkVtMI7StjqfiEw13q2WT6zd0AubBr1N6E9IPC8OAgH5uXx+GUZGSokAIKpFRZFvu1eZmZmM29ANWGUiocSMxeEEBg6CBiLvhm8rhEAA */
    id: 'appDeployment',
    context: { applicationId, messages: [] },
    initial: 'Idle',
    states: {
      Idle: {
        on: {
          'app-deployment.requested': {
            target: 'Discovering',
            actions: [
              assign(
                ({
                  event: {
                    payload: { applicationId, address, chainId },
                  },
                }) => ({
                  address,
                  chainId,
                  messages: [discoverSM({ applicationId, address, chainId })],
                }),
              ),
            ],
          },
        },
      },
      Discovering: {
        on: {
          'app-deployment.sc-discovered': {
            target: 'Confirming',
            actions: [
              assign(({ context: { applicationId } }) => ({
                messages: [confirmSM({ applicationId })],
              })),
            ],
          },
        },
      },
      Confirming: {
        on: {
          'app-deployment.sc-confirmed': {
            target: 'Registering',
            actions: [
              assign(({ context: { applicationId } }) => ({
                messages: [registerSM({ applicationId })],
              })),
            ],
          },
        },
      },
      Registering: {
        on: {
          'app-deployment.sc-registered': {
            target: 'Active',
            actions: [
              assign(({ context: { applicationId } }) => ({
                messages: [completed({ applicationId })],
              })),
            ],
          },
        },
      },
      Active: {
        type: 'final',
      },
    },
  });
}

export class AppDeployment {
  #actor: Actor<AppDeploymentMachine>;

  protected constructor(actor: Actor<AppDeploymentMachine>) {
    this.#actor = actor;
    this.#actor.start();
  }

  static init(applicationId: string) {
    const actor = createActor(factory({ applicationId }));
    return new AppDeployment(actor);
  }

  static fromSnapshot(snapshot: string) {
    const actor = createActor(factory({ applicationId: '' }), {
      snapshot: JSON.parse(snapshot) as Snapshot<unknown>,
    });
    return new AppDeployment(actor);
  }

  notify(event: AppDeploymentEvent): AppDeploymentMessage[] {
    this.#actor.send(event);
    return this.#actor.getSnapshot().context.messages;
  }

  get status() {
    return this.#actor.getSnapshot().value;
  }

  get applicationId() {
    return this.#actor.getSnapshot().context.applicationId;
  }

  get snapshot() {
    return JSON.stringify(this.#actor.getPersistedSnapshot());
  }
}
