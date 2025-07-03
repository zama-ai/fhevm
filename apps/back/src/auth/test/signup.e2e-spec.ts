import { EXPIRATION_TIME_IN_MILLISECONDS } from '#invitations/use-cases/create-invitation.use-case.js'
import {
  type GraphQlResponse,
  IntegrationManager,
  type User,
} from '#tests/integration.manager.js'
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

describe('signup', () => {
  const manager = new IntegrationManager({
    invitations: false,
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

  describe('given no other users exists with the same email', () => {
    let email: string

    beforeEach(() => {
      email = faker.internet.email()
    })

    describe('when signing up', () => {
      let result: GraphQlResponse<{ user: User }>
      let name: string
      let password: string

      beforeEach(async () => {
        name = faker.internet.username()
        password = faker.internet.password()
        result = await manager.auth.signup(
          {
            email,
            name,
            password,
          },
          { confirm: false },
        )
      })

      test('then it should create a new user', async () => {
        if (result.success) {
          expect(result.data.user.email).toBe(email)
          expect(result.data.user.name).toBe(name)
        } else {
          console.log(
            `signup should not fail: ${JSON.stringify(result.errors)}`,
          )
          expect(result.success).toBe(true)
        }
      })

      test('then the user receives a confirmation token', async () => {
        await vi.waitUntil(async () => {
          const event =
            await manager.getMessageFromOrchQueue('back:user:created')
          return event !== undefined
        })

        const event = await manager.getMessageFromOrchQueue('back:user:created')
        if (event) {
          expect(back.isBackEvent(event)).toBe(true)
          expect(event.type).toBe('back:user:created')
          expect((event.payload as any).email).toBe(email)
        } else {
          expect.fail('event is undefined')
        }
      })

      test('then the user cannot login before they confirm the email', async () => {
        const login = await manager.auth.login({
          email,
          password,
        })
        expect(login.success, 'login should fail before confirmation').toBe(
          false,
        )
      })
    })

    describe('when signing up and confirming the email', () => {
      let email: string
      let password: string

      beforeEach(async () => {
        email = faker.internet.email()
        password = faker.internet.password()
        const signup = await manager.auth.signup(
          {
            email,
            name: faker.internet.username(),
            password,
          },
          {
            confirm: false,
          },
        )
        if (!signup.success) {
          console.log(`failed to signup: ${JSON.stringify(signup)}`)
          expect(signup.success).toBe(true)
        }

        let token: string | undefined
        do {
          const event =
            await manager.getMessageFromOrchQueue('back:user:created')
          if (event) {
            token = (event.payload as any).token
          }
        } while (!token)

        const confirmEmail = await manager.auth.confirmEmail(token)
        if (!confirmEmail.success) {
          console.log(
            `failed to confirm email: ${JSON.stringify(confirmEmail)}`,
          )
          expect(confirmEmail.success).toBe(true)
        }
      })

      test('then the user can login', async () => {
        const login = await manager.auth.login({
          email,
          password,
        })
        if (!login.success) {
          console.log(`failed to login: ${JSON.stringify(login.errors)}`)
          expect(login.success).toBe(true)
        }
        expect(login.success, 'login should succeed').toBe(true)
      })
    })
  })

  describe('given a user signups with the same email', () => {
    let email: string
    let password: string

    beforeEach(async () => {
      email = faker.internet.email()
      password = faker.internet.password()
      const signup = await manager.auth.signup(
        {
          email,
          name: faker.internet.username(),
          password,
        },
        { confirm: true },
      )
      if (!signup.success) {
        console.log(`failed to signup: ${JSON.stringify(signup)}`)
        expect(signup.success).toBe(true)
      }
    })

    describe('when a new user signs up with the same email', () => {
      test('then it should fail', async () => {
        const signup = await manager.auth.signup({
          email,
          name: faker.internet.username(),
          password: faker.internet.password(),
        })
        expect(signup.success, 'signup should fail').toBe(false)
        if (!signup.success) {
          expect(signup.errors[0].message).toBe('Email already in use')
        }
      })
    })
  })
})
