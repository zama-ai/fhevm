import { IntegrationManager } from '#tests/integration.manager.js'
import { faker } from '@faker-js/faker'
import { back, operationName } from 'messages'
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
import { CumulativeStats } from '../domain/repositories/dapp.repository.js'

interface DappStats {
  id: string
  cumulative: CumulativeStats
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
          expect(cumulative.FheSub).toBe(0)
          expect(cumulative.FheMul).toBe(0)
          expect(cumulative.FheDiv).toBe(0)
          expect(cumulative.FheRem).toBe(0)
          expect(cumulative.FheBitAnd).toBe(0)
          expect(cumulative.FheBitOr).toBe(0)
          expect(cumulative.FheBitXor).toBe(0)
          expect(cumulative.FheShl).toBe(0)
          expect(cumulative.FheShr).toBe(0)
          expect(cumulative.FheRotl).toBe(0)
          expect(cumulative.FheRotr).toBe(0)
          expect(cumulative.FheEq).toBe(0)
          expect(cumulative.FheEqBytes).toBe(0)
          expect(cumulative.FheNe).toBe(0)
          expect(cumulative.FheNeBytes).toBe(0)
          expect(cumulative.FheGe).toBe(0)
          expect(cumulative.FheGt).toBe(0)
          expect(cumulative.FheLe).toBe(0)
          expect(cumulative.FheLt).toBe(0)
          expect(cumulative.FheMin).toBe(0)
          expect(cumulative.FheMax).toBe(0)
          expect(cumulative.FheNeg).toBe(0)
          expect(cumulative.FheNot).toBe(0)
          expect(cumulative.VerifyCiphertext).toBe(0)
          expect(cumulative.Cast).toBe(0)
          expect(cumulative.TrivialEncrypt).toBe(0)
          expect(cumulative.TrivialEncryptBytes).toBe(0)
          expect(cumulative.FheIfThenElse).toBe(0)
          expect(cumulative.FheRand).toBe(0)
          expect(cumulative.FheRandBounded).toBe(0)
        }
      })
    })
  })

  describe('given a dapp exists and it has some stats', () => {
    let dappId: string
    let token: string
    let teamId: string
    const chainId = 11155111 // Sepolia
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

      chainId = faker.number.int({ min: 1, max: 100_000 })
      // TODO: move to a GraphQL endpoint when implemented
      await manager.prismaClient.chain.create({
        data: {
          id: chainId,
          name: faker.string.alphanumeric(10),
          enabled: true,
        },
      })
      address = faker.string.hexadecimal({ length: 40 })
      const createDappResult = await manager.dapp.createDApp({
        token,
        teamId,
        name: faker.string.alphanumeric(10),
        chainId,
        address,
      })
      expect(createDappResult.success).toBe(true)
      if (createDappResult.success) {
        dappId = createDappResult.data.id
      }

      // Create some test stats using a factory
      const createFakeStatsMessages = (
        address: string,
        name: operationName,
      ) => ({
        payload: {
          requestId: faker.string.uuid(),
          chainId,
          address,
          events: [
            {
              name,
              timestamp: faker.date.past().toISOString(),
              externalRef: faker.string.alphanumeric(10),
            },
          ],
        },
        meta: {
          correlationId: faker.string.uuid(),
        },
      })
      const messages = [
        createFakeStatsMessages(address, 'FheAdd'),
        createFakeStatsMessages(address, 'FheAdd'),
        createFakeStatsMessages(address, 'FheBitAnd'),
      ]

      await Promise.all(
        messages.map(({ payload, meta }) =>
          manager.sendMessage(
            JSON.stringify(back.dappStatsAvailable(payload, meta)),
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
          const results = [
            ['total', 3],
            ['FheAdd', 2],
            ['FheBitAnd', 1],
            ['FheSub', 0],
            ['FheMul', 0],
            ['FheDiv', 0],
            ['FheRem', 0],
            ['FheBitOr', 0],
            ['FheBitXor', 0],
            ['FheShl', 0],
            ['FheShr', 0],
            ['FheRotl', 0],
            ['FheRotr', 0],
            ['FheEq', 0],
            ['FheEqBytes', 0],
            ['FheNe', 0],
            ['FheNeBytes', 0],
            ['FheGe', 0],
            ['FheGt', 0],
            ['FheLe', 0],
            ['FheLt', 0],
            ['FheMin', 0],
            ['FheMax', 0],
            ['FheNeg', 0],
            ['FheNot', 0],
            ['VerifyCiphertext', 0],
            ['Cast', 0],
            ['TrivialEncrypt', 0],
            ['TrivialEncryptBytes', 0],
            ['FheIfThenElse', 0],
            ['FheRand', 0],
            ['FheRandBounded', 0],
          ] as [keyof typeof cumulative, number][]
          for (const [field, value] of results) {
            expect(cumulative[field]).toBe(value)
          }
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

      const chainId = 11155111 // Sepolia

      const createDappResult = await manager.dapp.createDApp({
        token,
        teamId,
        name: faker.string.alphanumeric(10),
        chainId,
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
