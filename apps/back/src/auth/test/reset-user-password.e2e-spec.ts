import { Hash, Token } from '#auth/domain/entities/value-objects/index.js'
import { IntegrationManager } from '#tests/integration.manager.js'
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

describe('reset-user-password', () => {
  const manager = new IntegrationManager({
    invitations: false,
  })

  beforeEach(async () => {
    await manager.beforeEach()
  })

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30_000)

  beforeEach(async () => {
    await manager.beforeEach()
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given a user exists', () => {
    let email: string

    beforeEach(async () => {
      email = faker.internet.email()
      const signup = await manager.auth.signup(
        {
          email,
          password: faker.internet.password(),
        },
        {
          confirm: true,
        },
      )
      if (!signup.success) {
        console.log(`failed to signup: ${JSON.stringify(signup)}`)
        expect(signup.success).toBe(true)
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
          // The system generates at least two events:
          // 1. back:user:created
          // 2. back:user:password-reset:requested
          return (
            (await manager.getMessageFromOrchQueue(
              'back:password-reset:requested',
            )) !== undefined
          )
        })
        const event = await manager.getMessageFromOrchQueue(
          'back:password-reset:requested',
        )
        if (event) {
          expect(event.type).toBe('back:password-reset:requested')
          expect((event.payload as any).email).toBe(email)
          expect(Token.from((event.payload as any).token).isOk()).toBe(true)
        }
      })
    })
  })

  describe('given a user requested a password reset', () => {
    let email: string
    let token: string

    beforeEach(async () => {
      email = faker.internet.email()
      // Create a user
      const signup = await manager.auth.signup({
        email,
        password: faker.internet.password(),
      })
      if (!signup.success) {
        console.log(`failed to signup: ${JSON.stringify(signup)}`)
        expect(signup.success).toBe(true)
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
        const event = await manager.getMessageFromOrchQueue(
          'back:password-reset:requested',
        )
        return event !== undefined
      })
      const event = await manager.getMessageFromOrchQueue(
        'back:password-reset:requested',
      )
      if (event) {
        expect(event.type).toBe('back:password-reset:requested')
        expect((event.payload as any).email).toBe(email)
        expect(Token.from((event.payload as any).token).isOk()).toBe(true)
        token = (event.payload as any).token
      } else {
        expect.fail('event is undefined')
      }
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
        expect(login.success, 'failed to login').toBe(true)
      })

      test('then the previous token has been deleted', async () => {
        const hash = Hash.hash(Token.from(token).unwrap()).value
        // Note: sometime this test fails.
        // Sleep for 100ms to ensure the token has been deleted
        await new Promise(resolve => setTimeout(resolve, 100))
        await expect(
          manager.prismaClient.userToken.findUnique({
            where: {
              tokenHash: hash,
            },
          }),
        ).resolves.toBeNull()
      })

      test('then a password reset completed event has been generated', async () => {
        await vi.waitUntil(async () => {
          const event = await manager.getMessageFromOrchQueue(
            'back:password-reset:completed',
          )
          return event !== undefined
        })
        const event = await manager.getMessageFromOrchQueue(
          'back:password-reset:completed',
        )
        if (event) {
          expect(event.type).toBe('back:password-reset:completed')
          expect((event.payload as any).email).toBe(email)
        } else {
          expect.fail('event is undefined')
        }
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
          expect(
            requestPasswordReset.success,
            'failed to request password reset',
          ).toBe(true)
        }

        await vi.waitUntil(async () => {
          const events = await manager.getAllMessagesFromOrchQueue()

          return (
            events.filter(
              event => event.type === 'back:password-reset:requested',
            ).length > 1
          )
        })
        const events = await manager.getAllMessagesFromOrchQueue()
        const event = events
          .filter(event => event.type === 'back:password-reset:requested')
          .slice(-1)[0]
        if (event) {
          expect(event.type, 'wrong event type').toBe(
            'back:password-reset:requested',
          )
          newToken = (event.payload as any).token
        } else {
          expect.fail('event is undefined')
        }
      })

      test('then the previous token has been deleted', async () => {
        expect(newToken).not.toBe(token)
        const hash = Hash.hash(Token.from(newToken).unwrap()).value
        await expect(
          manager.prismaClient.userToken.findUnique({
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
