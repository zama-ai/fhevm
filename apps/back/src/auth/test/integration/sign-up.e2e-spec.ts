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
        ;({ token, user } = await manager.signup(
          invitation,
          faker.internet.username(),
          faker.internet.password(),
        ))
      })

      test('then it signs up the user', () => {
        expect(token).toBeDefined()
      })

      test('then it returns the user', () => {
        expect(user).toBeDefined()
        expect(user.email).toBe(email)
      })
    })
  })
  describe('given no invitation exists', () => {
    describe('when signing up', () => {
      let token: string
      let user: { email: string; name: string }

      beforeEach(async () => {
        ;({ token, user } = await manager.signup(
          faker.string.uuid(),
          faker.internet.username(),
          faker.internet.password(),
        ))
      })
      test('then it fails', async () => {
        expect(token).toBeFalsy()
      })

      test('then it does not create a user', () => {
        expect(user).toEqual({ email: '', name: '' })
      })
    })
  })
})
