import { assign, fromPromise, setup } from 'xstate'
import { Plan, PlanDetail, Product } from './types.js'

export interface ApplicationRegisteredService {
  listPlans: () => Promise<Plan[]>
  getPlanById(id: number): Promise<PlanDetail>
  listProducts(): Promise<Product[]>
  createAccessRequest: (input: {
    appId: number
    planId: number
    productIds: number[]
  }) => Promise<boolean>
}

interface ApplicationRegisteredContext {
  appId: number
  planIds?: number[]
  productIds?: number[]
  error?: string
}

type ApplicationRegisteredEvent = {
  type: 'START'
}

export function factory(service: ApplicationRegisteredService, appId: number) {
  return setup({
    types: {} as {
      context: ApplicationRegisteredContext
      events: ApplicationRegisteredEvent
    },
    actors: {
      listPlans: fromPromise(() => service.listPlans()),
      getDefaultPlan: fromPromise(
        async ({ input }: { input: { planIds: number[] } }) => {
          const plans = await Promise.all(
            input.planIds.map(service.getPlanById),
          )
          const defaultPlans = plans.filter(p => p.metadata['default'] === 'true');
          if (defaultPlans.length === 0) {
            throw new Error('Default plan not found');
          }
          if (defaultPlans.length > 1) {
            throw new Error('Multiple default plans found');
          }
          return defaultPlans[0];
        },
      ),
      listProducts: fromPromise(() => service.listProducts()),
      createAccessRequest: fromPromise(
        ({
          input,
        }: {
          input: { appId: number; planId: number; productIds: number[] }
        }) => service.createAccessRequest(input),
      ),
    },
    guards: {
      hasPlanAndProducts: ({ context: { planIds, productIds } }) =>
        planIds?.length === 1 && (productIds?.length ?? 0) > 0,
    },
  }).createMachine({
    /** @xstate-layout N4IgpgJg5mDOIC5QEMAOqA2BLAxsgLlgPYB2ASmFFrPmAE6QB0WEGYAxAMoAqAgmdwDaABgC6iUKiKwshUhJAAPRAFoATAGYAnIwCsW4WoAsutWq1qAjAA5LAdgA0IAJ6INd63rNqAbMO1WJnYAvsFOaJi4BMTklNS0DBCMAGZg+DgAFgAiBMjsEKRgjDQERRHYeHKxVDT0TKnp2bki4kggUjJVCsoIlr7CjD5GwvrWPhqW-kaOLm52aozDPr4a0z7rRhqh4egV0aQUNQn1aZk5+HkFJEUltIzlUVWH8XVJDWfNlq2S0rIx3Yg+nYBkMRloxhMpjNXAhdCNGAZlsDtLojJY0UZtiAHpUYs9aokUqcmhciY0AAoYZAkWBkzK8DAYSnU2D5QrMEgANyIAGsyrtHni4gSTo1zsg6RlmTTJQymVSaQgsFyiLjSC0WgoOn95G0emYdNYtJYfOjfOZhlonDC1HYfIxhNYPD5rNZdG6jEYtFicftqi9Ce8SRKg9LaUG5WH2PQ6EQ6IxMARknGALb3AVq-0it7E8WSsOyxlhpUqzMasRa35dPWINTCOyMOxGaxGcbaOw2Ky6a2IJ1GRgtjRqJ0+UZacY+jN+-HHHNi3L5hXh3NgZLIACuGHw0rZ1w53L56cimZnr0ledDS-Pq43W+lJe5ZbEmra2uroB6wIbmidEy0Wn0d1oVrO0HSdMZXXdVFdEnY9p2FWdzwXS8WWvNdN23BVozoWN40TfBkzoNNfSeBCzyDC9iQLCibww+9lUfP1y2+doq3+GsEBMTwbHbJsOyGUcewQO0dGEAwjFtCSNFNYRLFgvZSKOcjc2QqjYwgdccHwZdGkjdTNO03cigY3l+TgxSA1FD5SVDfStJ0+kizs7SH1VJjnwrV82N1D9axsRhLEsbRrD-d1VlbITTENJ1bXAjR9AseTBQOMjAxUmy1KIDT7MLJlnNZGM4wTKkCNTI8FKFJS0vnDKKXy3LyXy1yn1EF8fk6djfN6fsbCbLR3GGEx4o0DQhK0JtGG0Pogt8UxAq2MJsSnCzs0YHAGFKXgcBwOBYAoABHdc4HwIz91M8rkqzRD1rATbtt2g6jpoZr3Nazz2p1EgAV6WwG38QKQtdNENBCoSJntCTx3Bf9VkMT0kpPVKmBuu6dtgPawEO47sNw4qkzKkjKsspIUdoLa0YxrHnpMlq2tYjqfKUQFdGkgc7HBXRTGMVEPCE00NEYYcNEdaxhEMYbvSxEgsrgBRCZSqrIErBmvo4lQ5odU1rBWVthZdIT+xdYRPRC4dTCHcEEfgxWkhYNhlc+76VGmAXhC1nXxjd6wwY7RY3bUXR0X-J12Zgxb5au5SauQB33yZhAVHBBEBpBk10SHYcxoWGxjbRcdJn8KwrZWxCKNUikl1jzr4+d3QEV8SxOakiSWeA3p2cGUXNk5-rmwmTFw+WonVrL2rMmo4lI0rryVe+hZ6xBz1dDtZfW3MITfAbHPIbMfRPbDnZzOH0v0pDKirxo9C7wVKvGZ6T0dHGyYm0MOwm21je7AF2xPWHPpxrMAPQ+FUFbEyQmPKUV4rhgFvqrLq-4dDrDhHCR0nMjRWlmAgcw9o3aekCsvaSwdi7HyjtZM+dUsoGXgDPR2as4TJybKnU0mwzDe0wZ6AY81jStlRP+d0xDQEj1PvmeqEYnKUPsrAp2Cx+qMNsMwjOoNMGmAbPzFsVgxiBUsJLYBl1TzVTISIiR2lGDQKkRxYOA5ZImFHGLL+ahIraD0BbOwugfAWDrOYARkdCSkzAOTB6mMnr4HMV1RugdnHTTFqOQOXowbTARP7d0Vh1jGnrN4-RTBYCaQpqE+OdZgSLAQUFACBg36jUwf3AK7NgQbBMOCHwGSkZvGQFgNgEA8n6nrAML0-4Sn6HrF-MGLY-ZIjRPMVEgVQihCAA */
    id: 'applicationRegistered',
    initial: 'idle',
    context: { appId },
    states: {
      idle: {
        on: {
          START: {
            target: 'fetchData',
          },
        },
      },
      fetchData: {
        type: 'parallel',
        states: {
          fetchPlans: {
            initial: 'fetchAllPlans',

            states: {
              fetchAllPlans: {
                invoke: {
                  src: 'listPlans',
                  onDone: {
                    target: 'fetchDefaultPlan',
                    actions: assign({
                      planIds: ({ event }) => event.output.map(p => p.id),
                    }),
                  },
                  onError: {
                    target: '#applicationRegistered.failed',
                    actions: [
                      assign({
                        error: ({ event }) =>
                          `failed to fetch plans: ${event.error}`,
                      }),
                    ],
                  },
                },
              },
              fetchDefaultPlan: {
                invoke: {
                  src: 'getDefaultPlan',
                  input: ({ context: { planIds } }) => ({ planIds: planIds! }),
                  onDone: {
                    target: 'done',
                    actions: assign({
                      planIds: ({ event }) => [event.output.id],
                    }),
                  },
                  onError: {
                    target: '#applicationRegistered.failed',
                    actions: [
                      assign({
                        error: ({ event }) =>
                          `failed to get the default plan: ${event.error}`,
                      }),
                    ],
                  },
                },
              },
              done: {
                type: 'final',
              },
            },
          },
          fetchProducts: {
            initial: 'fetchAllProducts',
            states: {
              fetchAllProducts: {
                invoke: {
                  src: 'listProducts',
                  onDone: {
                    target: 'done',
                    actions: assign({
                      productIds: ({ event }) => event.output.map(p => p.id),
                    }),
                  },
                  onError: {
                    target: '#applicationRegistered.failed',
                    actions: [
                      assign({
                        error: ({ event }) =>
                          `failed to fetch products: ${event.error}`,
                      }),
                    ],
                  },
                },
              },
              done: {
                type: 'final',
              },
            },
          },
        },
        onDone: [
          {
            guard: 'hasPlanAndProducts',
            target: 'createAccessRequest',
          },
          {
            target: '#applicationRegistered.failed',
            actions: [assign({ error: 'no plan or products found' })],
          },
        ],
      },
      createAccessRequest: {
        invoke: {
          src: 'createAccessRequest',
          input: ({ context: { appId, planIds, productIds } }) => ({
            appId: appId!,
            planId: planIds![0],
            productIds: productIds!,
          }),
          onDone: {
            target: 'success',
          },
          onError: {
            target: 'failed',
            actions: [
              assign({
                error: ({ event }) =>
                  `failed to create access request: ${event.error}`,
              }),
            ],
          },
        },
      },
      success: {
        type: 'final',
      },
      failed: {
        type: 'final',
      },
    },
  })
}
