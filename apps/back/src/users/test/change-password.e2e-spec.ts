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
} from 'vitest'

describe('Change Password', () => {
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

  describe('given a user is logged in', () => {
    let email: string
    let password: string
    let token: string

    beforeEach(async () => {
      email = faker.internet.email()
      password = faker.internet.password()
      const login = await manager.auth.login(
        { email, password },
        { signup: true },
      )
      if (!login.success) {
        console.log(`failed to login: ${JSON.stringify(login)}`)
        expect(login.success).toBe(true)
      } else {
        token = login.data.token
      }
    })

    describe(`when changing the password`, () => {
      let newPassword: string
      beforeEach(async () => {
        newPassword = faker.internet.password()
        const changePassword = await manager.user.changePassword({
          token,
          oldPassword: password,
          newPassword,
        })
        if (!changePassword.success) {
          console.log(
            `failed to change password: ${JSON.stringify(changePassword)}`,
          )
          expect(changePassword.success).toBe(true)
        }
      })
      test('then it updates it', async () => {
        const login = await manager.auth.login({ email, password: newPassword })
        if (!login.success) {
          console.log(
            `failed to login with new password: ${JSON.stringify(login)}`,
          )
        }
        expect(login.success).toBe(true)
      })

      test("then the user can't login with the old password", async () => {
        const login = await manager.auth.login({ email, password })
        if (login.success) {
          console.log(
            `I could login with old password: ${JSON.stringify(login)}`,
          )
        }
        expect(login.success).toBe(false)
      })
    })
  })
})
