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
import { GraphQlResponse } from '#tests/setup.manager.js'

interface CumulativeDappStats {
  total: number
  FheAdd: number
  FheBitAnd: number
  FheIfThenElse: number
  FheLe: number
  FheOr: number
  FheSub: number
  TrivialEncrypt: number
  VerifyCiphertext: number
  FheMul: number
  FheDiv: number
}

interface DappStats {
  id: string
  cumulative: CumulativeDappStats
}

describe('get-dapp-cumulative-stats', () => {
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
        dappId = createDappResult.data.id
      }
    })

    describe('when a logged in user gets the dapp cumulative stats', () => {
      let result: GraphQlResponse<{ stats: DappStats }>

      beforeEach(async () => {
        result = await manager.dapp.getDappStats({
          token,
          dappId,
        })
      })

      test('then it should return zero counts for all operations', () => {
        expect(result.success).toBe(true)
        if (result.success && result.data) {
          const { cumulative } = result.data.stats
          expect(cumulative.total).toBe(0)
          expect(cumulative.FheAdd).toBe(0)
          expect(cumulative.FheBitAnd).toBe(0)
          expect(cumulative.FheIfThenElse).toBe(0)
          expect(cumulative.FheLe).toBe(0)
          expect(cumulative.FheOr).toBe(0)
          expect(cumulative.FheSub).toBe(0)
          expect(cumulative.TrivialEncrypt).toBe(0)
          expect(cumulative.VerifyCiphertext).toBe(0)
          expect(cumulative.FheMul).toBe(0)
          expect(cumulative.FheDiv).toBe(0)
        }
      })
    })
  })

  describe('given a dapp exists and it has some stats', () => {
    let dappId: string
    let token: string
    let teamId: string
    let address: string

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

      address = faker.string.hexadecimal({ length: 40 })
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

      // Create some test stats
      await manager.sendMessage(
        JSON.stringify(
          back.dappStatsAvailable(
            {
              requestId: faker.string.uuid(),
              chainId: LOCAL_FHEVM_CHAIN_ID,
              address,
              name: 'FheAdd',
              timestamp: faker.date.past().toISOString(),
              externalRef: faker.string.alphanumeric(10),
            },
            {
              correlationId: faker.string.uuid(),
            },
          ),
        ),
      )

      await manager.sendMessage(
        JSON.stringify(
          back.dappStatsAvailable(
            {
              requestId: faker.string.uuid(),
              chainId: LOCAL_FHEVM_CHAIN_ID,
              address,
              name: 'FheAdd',
              timestamp: faker.date.past().toISOString(),
              externalRef: faker.string.alphanumeric(10),
            },
            {
              correlationId: faker.string.uuid(),
            },
          ),
        ),
      )

      await manager.sendMessage(
        JSON.stringify(
          back.dappStatsAvailable(
            {
              requestId: faker.string.uuid(),
              chainId: LOCAL_FHEVM_CHAIN_ID,
              address,
              name: 'FheBitAnd',
              timestamp: faker.date.past().toISOString(),
              externalRef: faker.string.alphanumeric(10),
            },
            {
              correlationId: faker.string.uuid(),
            },
          ),
        ),
      )
    })

    describe('when a logged in user gets the dapp cumulative stats', () => {
      let result: GraphQlResponse<{ stats: DappStats }>

      beforeEach(async () => {
        await vi.waitUntil(async () => {
          const count = await manager.prismaClient.dappStat.count()
          return count === 3
        })

        result = await manager.dapp.getDappStats({
          token,
          dappId,
        })
      })

      test('then it should return the correct counts for each operation', () => {
        expect(result.success).toBe(true)
        if (result.success && result.data) {
          const { cumulative } = result.data.stats
          expect(cumulative.total).toBe(3)
          expect(cumulative.FheAdd).toBe(2)
          expect(cumulative.FheBitAnd).toBe(1)
          expect(cumulative.FheIfThenElse).toBe(0)
          expect(cumulative.FheLe).toBe(0)
          expect(cumulative.FheOr).toBe(0)
          expect(cumulative.FheSub).toBe(0)
          expect(cumulative.TrivialEncrypt).toBe(0)
          expect(cumulative.VerifyCiphertext).toBe(0)
          expect(cumulative.FheMul).toBe(0)
          expect(cumulative.FheDiv).toBe(0)
        }
      })
    })
  })

  describe('given a dapp has been deleted', () => {
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
      if (createDappResult.success) {
        dappId = createDappResult.data.id
        // TODO: move to a GraphQL endpoint when implemented
        await manager.prismaClient.dapp.update({
          data: { deletedAt: new Date() },
          where: { id: dappId },
        })
      } else {
        expect(createDappResult.success, 'it should succeed').toBe(true)
        console.log(`createDapp: ${JSON.stringify(createDappResult)}`)
      }
    })

    describe('when a logged in user gets the dapp cumulative stats', () => {
      let result: GraphQlResponse<{ stats: DappStats }>

      beforeEach(async () => {
        result = await manager.dapp.getDappStats({
          token,
          dappId,
        })
      })

      test('then it should return a Not Found error', () => {
        expect(result.success).toBe(false)
        if (!result.success && result.errors) {
          expect(result.errors.length).toBeGreaterThan(0)
          expect(result.errors[0].message).toContain('not found')
        }
      })
    })
  })
})
