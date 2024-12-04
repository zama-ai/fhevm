import { Actor, assign, createActor, setup, Snapshot } from 'xstate';
import { type AppDeploymentEvent, completed } from './app-deployment.events';
import { confirmSC, discoverSC, registerSC } from './app-deployment.commands';
import type { AppDeploymentMessage } from './app-deployment.messages';

type Context = {
  applicationId: string;
  deploymentId: string;
  address?: string;
  chainId?: string;
};

type AppDeploymentMachine = ReturnType<typeof factory>;

function factory({
  applicationId,
  deploymentId,
  notifyMessage,
}: Pick<Context, 'applicationId' | 'deploymentId'> & {
  notifyMessage: (message: AppDeploymentMessage) => void;
}) {
  return setup({
    types: {
      context: {} as Context,
      events: {} as AppDeploymentEvent,
    },
    guards: {
      isValid: ({ context, event }) =>
        context.applicationId === event.payload.applicationId &&
        context.deploymentId === event.payload.deploymentId,
    },
  }).createMachine({
    /** @xstate-layout N4IgpgJg5mDOIC5QEMAOqAiZUBsD2AngLZgB2ALgHQCSEOYAxGqgLQTb7FlUBOYAjgFc45SAG0ADAF1EoVHlgBLcorylZIAB6IAjACYAbJQkmTAZj2WArGasGAHABoQBRPZ2UALAE5f3wwDsBhIB-gEAvuHOzFi4hCQUlBiKsADGeABuYDyKpFBM6Gwc8dyUaWwp6Vl8EJIySCDySipqGtoIVt4SlAEBEvZWEp4SVno6Vk4ubvZe9gbBVjreATpDBnqR0eixnAlUAMJqAGaKPES5+cxFcVyJ5emkJ2fi0hpNyqrqDe2rVgGUYz0yx0fXWXUmrgQ9hmnjmCyWKzWGyiIBixVuVAASmAoClRDk8gVWOwbnsyqkWHxcbB8S96nIFB9Wt9dD5ut4dJ55rYzPYTHoAs5IfY9MZTBIDN4fJ4BYZIijSHh2PAGmjSdw3oyWl9QO0WAYhYh9WLTGZPEsDGZvLzPJtUdt0WTaPRNc1Pm1EDLDQhVp4eoY7PNPFaDJ5Rna1btSsk0plshdXUydVpPeNKGYQSLYQK+RJvAapj6hv6DIHQyGw2YIw71YlDo9Tuc8ontR6EF12Tp7LZ8zorYNPN7fSWy8H82HkVtMI7StjqfiEw13q2WT6zd0AubBr1N6E9IPC8OAgH5uXx+GUZGSokAIKpFRZFvu1eZmZmM29ANWGUiocSMxeEEBg6CBiLvhm8rhEAA */
    id: 'appDeployment',
    context: { applicationId, deploymentId, messages: [] },
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
                    payload: { applicationId, deploymentId, address, chainId },
                    $meta,
                  },
                }) => ({
                  applicationId,
                  deploymentId,
                  address,
                  chainId,
                  $meta,
                }),
              ),
              ({
                event: {
                  payload: { applicationId, deploymentId, address, chainId },
                  $meta,
                },
              }) =>
                notifyMessage(
                  discoverSC(
                    { applicationId, deploymentId, address, chainId },
                    $meta,
                  ),
                ),
            ],
          },
        },
      },
      Discovering: {
        on: {
          'app-deployment.sc-discovered': {
            guard: 'isValid',
            target: 'Confirming',
            actions: [
              ({
                event: {
                  payload: { applicationId, deploymentId },
                  $meta,
                },
              }) =>
                notifyMessage(
                  confirmSC({ applicationId, deploymentId }, $meta),
                ),
            ],
          },
        },
      },
      Confirming: {
        on: {
          'app-deployment.sc-confirmed': {
            guard: 'isValid',
            target: 'Registering',
            actions: [
              ({
                event: {
                  payload: { applicationId, deploymentId },
                  $meta,
                },
              }) =>
                notifyMessage(
                  registerSC({ applicationId, deploymentId }, $meta),
                ),
            ],
          },
        },
      },
      Registering: {
        on: {
          'app-deployment.sc-registered': {
            guard: 'isValid',
            target: 'Active',
            actions: [
              ({
                event: {
                  payload: { applicationId, deploymentId },
                  $meta,
                },
              }) =>
                notifyMessage(
                  completed({ applicationId, deploymentId }, $meta),
                ),
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

  constructor(
    {
      applicationId,
      deploymentId,
    }: { applicationId: string; deploymentId: string },
    snapshot?: string,
  ) {
    this.#actor = createActor(
      factory({
        applicationId,
        deploymentId,
        notifyMessage: this.notifyMessage,
      }),
      {
        snapshot: snapshot
          ? (JSON.parse(snapshot) as Snapshot<unknown>)
          : undefined,
      },
    );
    this.#actor.start();
  }

  private messages: AppDeploymentMessage[] = [];
  private notifyMessage = (message: AppDeploymentMessage) => {
    this.messages.push(message);
  };

  send(event: AppDeploymentEvent): AppDeploymentMessage[] {
    this.messages = [];
    this.#actor.send(event);
    return this.messages;
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
