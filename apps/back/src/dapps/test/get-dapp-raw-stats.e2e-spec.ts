import { DAppStats } from '#tests/dapp.manager.js'
import { IntegrationManager } from '#tests/integration.manager.js'
import { GraphQlResponse } from '#tests/setup.manager.js'
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

describe('get-dapp-raw-stats', () => {
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

      const chainId = 11155111
      const createDapp = await manager.dapp.createDApp({
        token,
        teamId,
        name: faker.string.alphanumeric(10),
        chainId,
        address: faker.string.hexadecimal({ length: 40 }),
      })
      if (createDapp.success) {
        dappId = createDapp.data.id
      } else {
        console.warn(`failed to create dapp: ${createDapp.errors[0].message}`)
        expect(createDapp.success).toBe(true)
      }
    })

    describe('when a logged in user gets the dapp stats', () => {
      let getDappRawStats: GraphQlResponse<DAppStats>

      beforeEach(async () => {
        getDappRawStats = await manager.dapp.getDappRawStats({
          token,
          dappId,
        })
      })

      test('then it should return an empty array', () => {
        if (getDappRawStats.success) {
          expect(getDappRawStats.data.rawStats.length).toBe(0)
        } else {
          console.warn(
            `failed to get dapp raw stats: ${getDappRawStats.errors[0].message}`,
          )
          expect(getDappRawStats.success).toBe(true)
        }
      })

      test('then it should emit a dapp stats requested event', async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getOrchQueueSize()
          return size > 0
        })

        const message = await manager.getMessageFromOrchQueue()
        const event = JSON.parse(message!)
        expect(back.isBackEvent(event)).toBe(true)
        expect(event.type).toBe('back:dapp:stats-requested')
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

      const createDapp = await manager.dapp.createDApp({
        token,
        teamId,
        name: faker.string.alphanumeric(10),
        chainId: 11155111, // Sepolia
        address: faker.string.hexadecimal({ length: 40 }),
      })
      if (createDapp.success) {
        dappId = createDapp.data.id
        // TODO: move to a GraphQL endpoint when implemented
        await manager.prismaClient.dapp.update({
          data: { deletedAt: new Date() },
          where: { id: dappId },
        })
      } else {
        console.warn(`failed to create dapp: ${createDapp.errors[0].message}`)
        expect(createDapp.success, 'it should succeed').toBe(true)
      }
    })

    describe('when a logged in user gets the dapp stats', () => {
      let getDappStats: GraphQlResponse<DAppStats>

      beforeEach(async () => {
        getDappStats = await manager.dapp.getDappRawStats({
          token,
          dappId,
        })
      })

      test(`then it should return a 'NotFound' error`, () => {
        if (!getDappStats.success) {
          expect(getDappStats.errors[0].message).toMatch('not found')
        } else {
          expect(getDappStats.success, 'it should fail').toBe(false)
        }
      })
    })
  })
})
