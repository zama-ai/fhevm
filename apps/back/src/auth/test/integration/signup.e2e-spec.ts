import { EXPIRATION_TIME_IN_MILLISECONDS } from '#invitations/use-cases/create-invitation.use-case.js'
import {
  type GraphQlResponse,
  IntegrationManager,
  type User,
} from '#tests/integration.manager.js'
import { faker } from '@faker-js/faker'
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
  vi,
} from 'vitest'

describe('signup', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30000)

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
      const request = await manager.auth.createInvitation(email)
      expect(request.success).toBe(true)
      if (request.success) {
        invitation = request.data.token
      }
    })

    describe('when signing up', () => {
      let token: string
      let user: User

      beforeEach(async () => {
        const result = await manager.auth.signup({
          invitation,
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
    })

    describe('when signing up twice', () => {
      let result: GraphQlResponse<{
        token: string
        user: User
      }>
      beforeEach(async () => {
        // first time
        result = await manager.auth.signup({
          invitation,
          name: faker.internet.username(),
          password: faker.internet.password(),
        })

        // second time
        result = await manager.auth.signup({
          invitation,
          name: faker.internet.username(),
          password: faker.internet.password(),
        })
      })

      test('then it fails', () => {
        expect(result.success).toBe(false)
        if (!result.success) {
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
        const result = await manager.auth.signup({
          invitation: faker.string.uuid(),
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
      const request = await manager.auth.createInvitation(
        faker.internet.email(),
      )
      expect(request.success).toBe(true)
      if (request.success) {
        invitation = request.data.token
      }
    })

    describe('when signing up', () => {
      let result: GraphQlResponse<{
        token: string
        user: User
      }>

      beforeEach(async () => {
        // Move forward in time
        vi.useFakeTimers()
        vi.setSystemTime(Date.now() + EXPIRATION_TIME_IN_MILLISECONDS + 1)
        result = await manager.auth.signup({
          invitation,
          name: faker.internet.username(),
          password: faker.internet.password(),
        })
      })

      afterEach(() => {
        // Reset time
        vi.useRealTimers()
      })

      test('then it fails', async () => {
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.errors?.length).toBe(1)
          expect(result.errors?.[0].message).toContain('invalid token')
        }
      })
    })
  })
})
