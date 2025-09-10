import { assign, fromPromise, setup } from 'xstate'
import { NewOrganisation, User } from './types.js'

export interface UserRegisteredService {
  createOrganisation: (name: string) => Promise<NewOrganisation>
  updateUser: (
    userId: number,
    payload: Partial<Omit<User, 'id'>>,
  ) => Promise<User>
}

interface UserRegisteredContext {
  user: User
  organisation?: NewOrganisation
  error?: string
}

type UserRegisteredEvent = { type: 'START' }

export function factory(service: UserRegisteredService, user: User) {
  return setup({
    types: {} as {
      context: UserRegisteredContext
      events: UserRegisteredEvent
    },
    actors: {
      createOrganisation: fromPromise(
        async ({ input }: { input: { name: string } }) => {
          const org = await service.createOrganisation(input.name)
          return org
        },
      ),
      assignUserToOrganisation: fromPromise(
        async ({ input }: { input: { user: User; org: NewOrganisation } }) => {
          await service.updateUser(input.user.id, {
            orgId: input.org.id,
            teamIds: [input.org.teams[0].id],
          })
        },
      ),
    },
    guards: {
      belongsToDefaultUser: ({ context: { user } }) => user?.orgId === 1,
    },
  }).createMachine({
    id: 'userRegistered',
    initial: 'idle',
    context: { user },
    states: {
      idle: {
        on: {
          START: {
            target: 'started',
          },
        },
      },
      started: {
        always: [
          {
            guard: 'belongsToDefaultUser',
            target: 'creatingOrganisation',
          },
          {
            target: 'success',
          },
        ],
      },
      creatingOrganisation: {
        invoke: {
          id: 'createOrganisation',
          src: 'createOrganisation',
          input: ({ context: { user } }) => ({ name: `${user?.name}'s Org` }),
          onDone: {
            target: 'assigningUserToOrganisation',
            actions: assign({ organisation: ({ event }) => event.output }),
          },
          onError: {
            target: 'failed',
            actions: assign({
              error: ({ event }) => `failed to create org: ${event.error}`,
            }),
          },
        },
      },
      assigningUserToOrganisation: {
        invoke: {
          id: 'assignUserToOrganisation',
          src: 'assignUserToOrganisation',
          input: ({ context: { user, organisation } }) => ({
            user: user!,
            org: organisation!,
          }),
          onDone: {
            target: 'success',
          },
          onError: {
            target: 'failed',
            actions: assign({
              error: ({ event }) =>
                `failed to assign user to org: ${event.error}`,
            }),
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
