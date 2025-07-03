import {
  GraphQlResponse,
  IntegrationManager,
  User,
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
} from 'vitest'

describe('login', () => {
  const manager = new IntegrationManager({
    invitations: false,
  })

  beforeAll(async () => {
    await manager.beforeAll()
  }, 30000)

  beforeEach(async () => {
    await manager.beforeEach()
  })

  afterAll(async () => {
    await manager.afterAll()
  })

  afterEach(async () => {
    await manager.afterEach()
  })

  describe('given a confirmed user exists', () => {
    let email: string
    let password: string

    beforeEach(async () => {
      email = faker.internet.email()
      password = faker.internet.password()

      const signup = await manager.auth.signup({
        email,
        name: faker.internet.username(),
        password,
      })
      if (!signup.success) {
        console.log(`failed to signup: ${JSON.stringify(signup)}`)
        expect(signup.success).toBe(true)
      }
    })

    describe('when logging in', () => {
      let token: string
      let user: User

      beforeEach(async () => {
        const login = await manager.auth.login({ email, password })
        if (login.success) {
          token = login.data.token
          user = login.data.user
        } else {
          console.log(`failed to login: ${JSON.stringify(login.errors)}`)
          expect(login.success, 'login should succeed').toBe(true)
        }
      })

      test('then it returns a token', () => {
        expect(token).toEqual(expect.any(String))
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
  describe('given a not confirmed user exists', () => {
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
        { confirm: false },
      )
      if (!signup.success) {
        console.log(`failed to signup: ${JSON.stringify(signup.errors)}`)
        expect(signup.success).toBe(true)
      }
    })

    describe('when logging in', () => {
      test('then it should return unauthorized', async () => {
        const result = await manager.auth.login({ email, password })
        if (result.success) {
          expect(result.success, 'login should fail').toBe(false)
        } else {
          expect(result.errors[0].message).toBe('Unauthorized')
        }
      })
    })
  })

  describe('given a user has been deleted', () => {
    let email: string
    let password: string

    beforeEach(async () => {
      email = faker.internet.email()
      password = faker.internet.password()

      const signup = await manager.auth.signup({
        email,
        name: faker.internet.username(),
        password,
      })
      if (!signup.success) {
        console.log(`failed to signup: ${JSON.stringify(signup.errors)}`)
        expect(signup.success).toBe(true)
      }
      // TODO: use a GraphQL endpoint when implemented
      await manager.prismaClient.user.update({
        where: { email },
        data: { deletedAt: new Date() },
      })
    })

    describe('when logging in', () => {
      let login: GraphQlResponse<{
        user: User
        token: string
      }>

      beforeEach(async () => {
        login = await manager.auth.login({ email, password })
      })

      test('should return an Unauthorized error', () => {
        if (!login.success) {
          expect(login.errors[0].message).toMatch('Unauthorized')
        } else {
          expect(login.success, 'login should fail').toBe(false)
        }
      })
    })
  })

  describe(`given a user doesn't exist`, () => {
    let email: string
    let password: string

    beforeEach(async () => {
      email = faker.internet.email()
      password = faker.internet.password()
    })

    describe('when logging in', () => {
      let login: GraphQlResponse<{
        user: User
        token: string
      }>

      beforeEach(async () => {
        login = await manager.auth.login({ email, password })
      })

      test('should return an Unauthorized error', () => {
        if (!login.success) {
          expect(login.errors[0].message).toMatch('Unauthorized')
        } else {
          expect(login.success, 'login should fail').toBe(false)
        }
      })
    })
  })
})
