import { EXPIRATION_TIME_IN_MILLISECONDS } from '@/invitations/use-cases/create-invitation.use-case'
import {
  type GraphQlResponse,
  IntegrationManager,
  type User,
} from '@/tests/integration.manager'
import { faker } from '@faker-js/faker'
import {
  afterAll,
  afterEach,
  assert,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from 'vitest'

describe('sign-up', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given an invitation exists', () => {
    let invitation: string
    let email: string

    beforeEach(async () => {
      email = faker.internet.email()
      invitation = await manager.createInvitation(email)
    })

    describe('when signing up', () => {
      let token: string
      let user: User

      beforeEach(async () => {
        const result = await manager.signup({
          token: invitation,
          name: faker.internet.username(),
          password: faker.internet.password(),
        })
        if (result.success) {
          ;({ token, user } = result.data)
        }
      })

      test('then it signs up the user', () => {
        expect(token, 'Token should be defined after signing up').toBeDefined()
      })

      test('then it returns the user', () => {
        expect(user, 'User should be defined after signing up').toBeDefined()
        expect(user.email).toBe(email)
      })

      test('then it creates a default team', () => {
        expect(user.teams.length).toBe(1)
      })

      test('then it fails to sign up with the same invitation twice', async () => {
        const result = await manager.signup({
          token: invitation,
          name: faker.internet.username(),
          password: faker.internet.password(),
        })
        if (result.success) {
          assert.fail('Should not be able to sign up twice')
        } else {
          expect(result.errors[0].message).toContain('invalid token')
        }
      })
    })
  })

  describe('given no invitation exists', () => {
    describe('when signing up', () => {
      let token: string
      let user: { email: string; name: string }

      beforeEach(async () => {
        const result = await manager.signup({
          token: faker.string.uuid(),
          name: faker.internet.username(),
          password: faker.internet.password(),
        })
        if (result.success) {
          ;({ token, user } = result.data)
        }
      })
      test('then it fails', async () => {
        expect(token).toBeUndefined()
      })

      test('then it does not return a user', () => {
        expect(user).toBeUndefined()
      })
    })
  })

  describe('given an expired invitation', () => {
    let invitation: string
    beforeEach(async () => {
      invitation = await manager.createInvitation(faker.internet.email())
    })

    describe('when signing up', () => {
      let result: GraphQlResponse<{
        token: string
        user: User
      }>

      beforeEach(async () => {
        // Move forward in time
        vi.setSystemTime(EXPIRATION_TIME_IN_MILLISECONDS + 1)
        result = await manager.signup({
          token: invitation,
          name: faker.internet.username(),
          password: faker.internet.password(),
        })
      })

      afterEach(() => {
        // Reset time
        vi.useRealTimers()
      })

      test('then it fails', async () => {
        if (result.success) {
          assert.fail('Should not be able to sign up')
        } else {
          expect(result.success).toBe(false)

          expect(result.errors?.length).toBe(1)
          expect(result.errors?.[0].message).toContain('not found')
        }
      })
    })
  })
})
