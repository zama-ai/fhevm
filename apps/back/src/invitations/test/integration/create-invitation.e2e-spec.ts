import {
  afterAll,
  afterEach,
  beforeAll,
  beforeEach,
  describe,
  expect,
  test,
} from 'vitest'
import { faker } from '@faker-js/faker'
import { IntegrationManager } from '#tests/integration.manager.js'

describe('createInvitation', () => {
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

  describe('given no invitation exists', () => {
    describe('when creating an invitation', () => {
      let email: string
      let token: string

      beforeEach(async () => {
        email = faker.internet.email()
        const request = await manager.auth.createInvitation(email)
        expect(request.success).toBe(true)
        if (request.success) {
          token = request.data.token
        }
      })

      test('then it returns a new invitation token', () => {
        expect(token).toBeDefined()
      })

      test('then it creates a new invitation with the right email', async () => {
        const request = await manager.auth.getInvitation(token)
        expect(request.success).toBe(true)
        if (request.success) {
          expect(request.data.email).toBe(email)
        }
      })
    })
  })

  describe('given an invitation exists', () => {
    let email: string
    let token: string

    beforeEach(async () => {
      email = faker.internet.email()
      const request = await manager.auth.createInvitation(email)
      if (request.success) {
        token = request.data.token
      } else {
        console.log(`failed to create invitation: ${JSON.stringify(request)}`)
        expect(request.success).toBe(true)
      }
    })

    test('then it fails due a duplicated email error', async () => {
      const request = await manager.auth.createInvitation(email)
      expect(request.success).toBe(false)
      if (!request.success) {
        expect(request.errors).toBeDefined()
        expect(request.errors.length).toBeGreaterThan(0)
        expect(request.errors[0].message).toBe('Email already used')
      }
    })
  })
})
