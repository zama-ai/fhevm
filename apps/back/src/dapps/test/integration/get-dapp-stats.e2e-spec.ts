import { DAppStats } from '#tests/dapp.manager.js'
import { IntegrationManager } from '#tests/integration.manager.js'
import { GraphQlResponse } from '#tests/setup.manager.js'
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

describe('get-dapp-stats', () => {
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

  describe('given a dapp exists and it has no stats', () => {
    let dappId: string
    let token: string
    let teamId: string

    beforeEach(async () => {
      const result = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      expect(result.success, 'Failed to login the user').toBe(true)
      if (result.success) {
        token = result.data.token
        teamId = result.data.user.teams[0].id
      }

      const createDappResult = await manager.dapp.createDApp({
        token,
        teamId,
        name: faker.string.alphanumeric(10),
        address: faker.string.hexadecimal({ length: 40 }),
      })
      expect(createDappResult.success).toBe(true)
      if (createDappResult.success) {
        dappId = createDappResult.data.dapp.id
      }
    })

    describe('when a logger user gets the dapp stats', () => {
      let result: GraphQlResponse<DAppStats>

      beforeEach(async () => {
        result = await manager.dapp.getDappStats({
          token,
          dappId,
        })
      })

      test('then it should return an empty array', () => {
        expect(result.success).toBe(true)
        if (result.success) {
          expect(result.data.stats.length).toBe(0)
        }
      })
    })
  })
})
