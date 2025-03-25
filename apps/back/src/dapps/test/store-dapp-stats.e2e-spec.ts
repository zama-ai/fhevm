import { IntegrationManager } from '#tests/integration.manager.js'
import { faker } from '@faker-js/faker'
import { back } from 'messages'
import { LOCAL_FHEVM_CHAIN_ID } from 'utils'
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

describe('store dapp stats', () => {
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

  describe('given it exists a dapp', () => {
    let dappId: string
    let address: string
    let token: string

    beforeEach(async () => {
      let teamId: string = ''
      address = faker.string.hexadecimal({ length: 40 })
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
        address,
      })
      expect(createDappResult.success).toBe(true)
      if (createDappResult.success) {
        dappId = createDappResult.data.id
      }
    })

    describe('when we receive a dapp stats available event', () => {
      beforeEach(async () => {
        await manager.sendMessage(
          JSON.stringify(
            back.dappStatsAvailable(
              {
                requestId: faker.string.uuid(),
                chainId: LOCAL_FHEVM_CHAIN_ID,
                address,
                events: [
                  {
                    name: 'FheAdd',
                    timestamp: faker.date.past().toISOString(),
                    externalRef: faker.string.alphanumeric(10),
                  },
                ],
              },
              {
                correlationId: faker.string.uuid(),
              },
            ),
          ),
        )
      })

      test('then it stores the dapp stats', async () => {
        await vi.waitUntil(async () => {
          const count = await manager.prismaClient.dappStat.count()
          return count > 0
        })
        const res = await manager.dapp.getDappRawStats({ token, dappId })
        if (res.success) {
          expect(res.data.id, 'Wrong dapp id').toBe(dappId)
          expect(res.data.stats.length, 'Wrong stats count').toBe(1)
          expect(res.data.stats[0].name).toBe('FheAdd')
        } else {
          console.log(`res: ${JSON.stringify(res)}`)
          expect(res.success, 'Failed to fetch stats').toBe(true)
        }
      })
    })
  })
})
