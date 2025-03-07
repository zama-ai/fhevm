import {
  DApp,
  GraphQlResponse,
  IntegrationManager,
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

describe('create-dapp', () => {
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

  describe('given a user is logged in', () => {
    let token: string
    let teamId: string

    beforeEach(async () => {
      const result = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      if (result.success) {
        token = result.data.token
        teamId = result.data.user.teams[0].id
      }
    })

    describe('when creating a dapp', () => {
      let dapp: DApp | undefined

      beforeEach(async () => {
        const result = await manager.dapp.createDApp({
          token,
          teamId,
          name: faker.string.alphanumeric(10),
        })
        if (result.success) {
          dapp = result.data
        }
      })

      test('then it creates a DRAFT dapp', () => {
        expect(dapp).toBeDefined()
        expect(dapp?.status).toBe('DRAFT')
      })

      test('then it associates the dapp with the right team', () => {
        expect(dapp?.team.id).toBe(teamId)
      })
    })
  })

  describe('given a user is not logged in', () => {
    describe('when creating a dapp', () => {
      let result: GraphQlResponse<DApp>

      beforeEach(async () => {
        result = await manager.dapp.createDApp({
          token: faker.string.uuid(),
          teamId: faker.string.uuid(),
          name: faker.string.alphanumeric(10),
        })
      })

      test('then it raises unauthorized error', () => {
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.errors).toBeDefined()
          expect(result.errors.length).toBeGreaterThan(0)
          expect(result.errors[0].message).toBe('Unauthorized')
        }
      })
    })
  })
})
