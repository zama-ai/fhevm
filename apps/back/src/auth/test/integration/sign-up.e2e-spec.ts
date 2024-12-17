import { IntegrationManager } from '@/tests/integration.manager'
import { faker } from '@faker-js/faker'
import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
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
      let user: { email: string; name: string }

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
})
