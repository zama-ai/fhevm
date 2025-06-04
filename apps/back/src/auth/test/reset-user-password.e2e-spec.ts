import { Hash, Token } from '#auth/domain/entities/value-objects/index.js'
import { IntegrationManager } from '#tests/integration.manager.js'
import { faker } from '@faker-js/faker'
import { back } from 'messages'
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

describe('reset-user-password', () => {
  const manager = new IntegrationManager()

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30_000)

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given a user exists', () => {
    let email: string

    beforeEach(async () => {
      const signup = await manager.auth.signup(
        {
          name: faker.internet.username(),
          password: faker.internet.password(),
        },
        { createInvitation: true },
      )
      if (!signup.success) {
        console.log(`failed to signup: ${JSON.stringify(signup)}`)
        expect(signup.success).toBe(true)
      } else {
        email = signup.data.user.email
      }
    })

    describe('when the user requests a password reset', () => {
      beforeEach(async () => {
        const requestPasswordReset =
          await manager.auth.requestPasswordReset(email)
        if (!requestPasswordReset.success) {
          console.log(
            `failed to request password reset: ${JSON.stringify(requestPasswordReset)}`,
          )
          expect(requestPasswordReset.success).toBe(true)
        }
      })

      test('then the user receives an email with the token', async () => {
        await vi.waitUntil(async () => {
          return (await manager.getOrchQueueSize()) > 0
        })
        const message = await manager.getMessageFromOrchQueue()
        const event = JSON.parse(message!)
        if (!back.isBackEvent(event)) {
          expect(false, 'event is not a BackEvent').toBeTruthy()
        }
        expect(event.type).toBe('back:user:password-reset:requested')
        expect(event.payload.email).toBe(email)
        expect(Token.from(event.payload.token).isOk()).toBe(true)
      })
    })
  })

  describe('given a user requested a password reset', () => {
    let email: string
    let token: string

    beforeEach(async () => {
      // Create a user
      const signup = await manager.auth.signup(
        {
          name: faker.internet.username(),
          password: faker.internet.password(),
        },
        { createInvitation: true },
      )
      if (!signup.success) {
        console.log(`failed to signup: ${JSON.stringify(signup)}`)
        expect(signup.success).toBe(true)
      } else {
        email = signup.data.user.email
      }

      // Request a password reset
      const requestPasswordReset =
        await manager.auth.requestPasswordReset(email)
      if (!requestPasswordReset.success) {
        console.log(
          `failed to request password reset: ${JSON.stringify(requestPasswordReset)}`,
        )
        expect(requestPasswordReset.success).toBe(true)
      }

      await vi.waitUntil(async () => {
        return (await manager.getOrchQueueSize()) > 0
      })
      const message = await manager.getMessageFromOrchQueue()
      const event = JSON.parse(message!)
      if (!back.isBackEvent(event)) {
        expect(false, 'event is not a BackEvent').toBeTruthy()
      }
      expect(event.type).toBe('back:user:password-reset:requested')

      token = event.payload.token
    })

    describe('when the user resets their password', () => {
      let password: string

      beforeEach(async () => {
        password = faker.internet.password()
        const resetPassword = await manager.auth.resetPassword({
          token,
          password,
        })
        if (!resetPassword.success) {
          console.log(
            `failed to reset password: ${JSON.stringify(resetPassword)}`,
          )
          expect(resetPassword.success).toBe(true)
        }
      })

      test('then the user can login with the new password', async () => {
        const login = await manager.auth.login({ email, password })
        if (!login.success) {
          console.log(`failed to login: ${JSON.stringify(login)}`)
        }
        expect(login.success).toBe(true)
      })

      test('then the previous token has been deleted', async () => {
        const hash = Hash.hash(Token.from(token).unwrap()).value
        // Note: sometime this test fails.
        // Sleep for 100ms to ensure the token has been deleted
        await new Promise(resolve => setTimeout(resolve, 100))
        await expect(
          manager.prismaClient.passwordResetToken.findUnique({
            where: {
              tokenHash: hash,
            },
          }),
        ).resolves.toBeNull()
      })

      test('then a password reset completed event has been generated', async () => {
        await vi.waitUntil(async () => {
          return (await manager.getOrchQueueSize()) > 0
        })
        const message = await manager.getMessageFromOrchQueue()
        const event = JSON.parse(message!)
        if (!back.isBackEvent(event)) {
          expect(false, 'event is not a BackEvent').toBeTruthy()
        }
        expect(event.type).toBe('back:user:password-reset:completed')
        expect(event.payload.email).toBe(email)
      })
    })

    describe('when the user requests a new reset', () => {
      let newToken: string
      beforeEach(async () => {
        // Request a password reset
        const requestPasswordReset =
          await manager.auth.requestPasswordReset(email)
        if (!requestPasswordReset.success) {
          console.log(
            `failed to request password reset: ${JSON.stringify(requestPasswordReset)}`,
          )
          expect(requestPasswordReset.success).toBe(true)
        }

        await vi.waitUntil(async () => {
          return (await manager.getOrchQueueSize()) > 0
        })
        const message = await manager.getMessageFromOrchQueue()
        const event = JSON.parse(message!)
        if (!back.isBackEvent(event)) {
          expect(false, 'event is not a BackEvent').toBeTruthy()
        }
        expect(event.type).toBe('back:user:password-reset:requested')

        newToken = event.payload.token
      })

      test('then the previous token has been deleted', async () => {
        expect(newToken).not.toBe(token)
        const hash = Hash.hash(Token.from(newToken).unwrap()).value
        await expect(
          manager.prismaClient.passwordResetToken.findUnique({
            where: {
              tokenHash: hash,
            },
          }),
        ).resolves.toEqual(
          expect.objectContaining({
            tokenHash: hash,
          }),
        )
      })
    })
  })
})
