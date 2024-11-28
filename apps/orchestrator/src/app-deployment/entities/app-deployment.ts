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
    /** @xstate-layout N4IgpgJg5mDOIC5QEMAOqAiZUBsD2AngLZgB2ALgHQCSEOYAxGqgLQTb7FlUBOYAjgFc45SAG0ADAF1EoVHlgBLcorylZIAB6IAjACYAbJQkmTAZj2WArGasGAHABoQBRPZ2UALAE5f3wwDsBhIB-gEAvuHOzFi4hCQUlBiKsADGeABuYDyKpFBM6Gwc8dyUsERsKelZfBCSMkgg8koqahraCFZ69pR6Zp5mBgYBlhJm3s6uCO5efv7DwaF6EVEgMcVciQDCagBmijxEufnMRXGbVOUs6aT7h+LSGs3KquqNHd7Bxt5mARYB9j03k8IUmbg8Pj8gUWYUi0XQsU4CSoACUwFAUqIcnkCqx2OdkWUKnwMbAsQ8GnIFC82u9dGZ7AFKHMRvZ3PZ+mYwdMIXNoSFYXCQKQ8Ox4I11gTuE9qa03qAOiwDNylcZTOqNSEhZKkaVaPQZS1Xu1EJ49NydBJPJQRgY7EMBp9PF1tQiNoTkmlMtljoaafKtIgGT0JD8Ap4QRI+r8DJ4LVabYZ7bHxrGXasdSVtnsDkc8n65SbpnorL1Ie5hnpPMEuS5dAnbcnHbGfK7MO7SmjSVjfY1noW6Qh9FYPBIDFWRjp7FYJHYrPHrY2himna2M26pYkAIKpFRZAvGwc6YbeShT8MOHRmfQ6M3xsxeII2CzXqMGbz2SKRIA */
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
