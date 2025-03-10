import { IntegrationManager, User } from '#tests/integration.manager.js'
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

describe('login', () => {
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

  describe('given a user exists', () => {
    let email: string
    let password: string

    beforeEach(async () => {
      email = faker.internet.email()
      password = faker.internet.password()

      await manager.auth.signup(
        {
          name: faker.internet.username(),
          password,
        },
        { createInvitation: true, email },
      )
    })

    describe('when logging in', () => {
      let token: string
      let user: User

      beforeEach(async () => {
        const result = await manager.auth.login({ email, password })
        if (result.success) {
          ;({ token, user } = result.data)
        }
      })

      test('then it returns a token', () => {
        expect(token).not.toBeFalsy()
      })

      test('then it returns the user', () => {
        expect(
          user,
          'The user should be defined after logging in',
        ).toBeDefined()
        expect(user.email, 'The email should be the expected one').toBe(email)
        expect(user.name, 'The name should be defined').toBeDefined()
      })
    })

    describe('when the credentials are invalid', () => {
      let error: string

      beforeEach(async () => {
        const result = await manager.auth.login({
          email,
          password: faker.internet.password(),
        })
        if (!result.success) {
          error = result.errors[0].message
        } else {
          expect(result.success, 'login should fail').toBe(false)
        }
      })

      test('should return an Unauthorized error', () => {
        expect(error).toBe('Unauthorized')
      })
    })
  })
})
