import {
  DeployDappResult,
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
  vi,
} from 'vitest'
import { DAppStatus } from '#dapps/domain/entities/dapp.js'
import { back } from 'messages'

describe('deploy-dapp', () => {
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

  describe('given a dapp is created', () => {
    let token: string
    let teamId: string
    let dappId: string

    beforeEach(async () => {
      const result = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      expect(result.success, 'Failed to login the user').toBe(true)
      if (result.success) {
        token = result.data.token
        teamId = result.data.user.teams[0].id

        const dappResult = await manager.dapp.createDApp({
          token,
          teamId,
          name: faker.string.alphanumeric(10),
        })
        expect(dappResult.success, 'Failed to create dApp').toBe(true)
        if (dappResult.success) {
          dappId = dappResult.data.id
        }
      }
    })

    describe('when deploying a dapp', () => {
      let result: GraphQlResponse<DeployDappResult>

      beforeEach(async () => {
        result = await manager.dapp.deployDApp({
          token,
          dappId,
        })
      })

      test('then it fails due missing address', () => {
        expect(result.success, 'dApp should not be deployed').toBe(false)
        if (!result.success) {
          expect(result.errors, 'should returns a list of errors').toBeDefined()
          expect(result.errors.length).toBeGreaterThan(0)
          expect(result.errors[0].message).contain('missing dApp address')
        }
      })
    })

    describe('when deploying a dapp after updating the dapp address', () => {
      let status: DAppStatus

      beforeEach(async () => {
        const dappResult = await manager.dapp.updateDApp({
          token,
          dappId,
          address: faker.string.hexadecimal({ length: 40 }),
        })
        expect(dappResult.success, 'Failed to update dApp address').toBe(true)
        const result = await manager.dapp.deployDApp({
          token,
          dappId,
        })
        expect(result.success).toBe(true)
        if (result.success) {
          status = result.data.status
        }
      })

      test('then the dapp status is updated to "DEPLOYING"', () => {
        expect(status).toBe('DEPLOYING')
      })
    })
  })

  describe('given a dapp is deployed', () => {
    let token: string
    let teamId: string
    let dappId = ''

    beforeEach(async () => {
      const result = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      expect(result.success, 'Failed to login the user').toBe(true)
      if (result.success) {
        token = result.data.token
        teamId = result.data.user.teams[0].id

        const dappResult = await manager.dapp.createDApp({
          token,
          teamId,
          name: faker.string.alphanumeric(10),
          address: faker.string.hexadecimal({ length: 40 }),
        })
        expect(dappResult.success).toBe(true)
        if (dappResult.success) {
          dappId = dappResult.data.id
          const result = await manager.dapp.deployDApp({
            token,
            dappId,
          })
          expect(result.success).toBe(true)
        }
      }
    })

    describe.each([
      'back:dapp:validation:requested',
      'back:dapp:validation:confirmed',
      'back:dapp:validation:failed',
    ] satisfies back.BackEvent['type'][])('when receiving `%s` event', type => {
      let status: DAppStatus
      switch (type) {
        case 'back:dapp:validation:requested':
          status = 'DEPLOYING'
          break
        case 'back:dapp:validation:confirmed':
          status = 'LIVE'
          break
        case 'back:dapp:validation:failed':
          status = 'FAILED'
          break
        default:
          status = 'DEPLOYING'
      }

      beforeEach(async () => {
        const message = genMessage(type, dappId)
        if (message) {
          await manager.sendMessage(JSON.stringify(message))
        }
      })

      test(`then the dapp status should be "${status}"`, async () => {
        await vi.waitUntil(async () => {
          const size = await manager.getQueueSize()
          return size === 0
        })

        const result = await manager.dapp.getDapp({
          token,
          dappId,
        })
        if (!result.success) {
          console.log(result)
        }
        expect(result.success).toBe(true)
        if (result.success) {
          expect(result.data.status).toBe(status)
        }
      })
    })
  })
})

function genMessage(
  type: back.BackEvent['type'],
  dappId: string,
): back.BackEvent | undefined {
  switch (type) {
    case 'back:dapp:validation:requested':
      return back.dappValidationRequested(
        {
          requestId: faker.string.uuid(),
          dAppId: dappId,
          chainId: '1',
          address: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      )
    case 'back:dapp:validation:confirmed':
      return back.dappValidationConfirmed(
        {
          requestId: faker.string.uuid(),
          dAppId: dappId,
          owner: faker.string.hexadecimal({ length: 40 }),
        },
        { correlationId: faker.string.uuid() },
      )
    case 'back:dapp:validation:failed':
      return back.dappValidationFailed(
        {
          requestId: faker.string.uuid(),
          dAppId: dappId,
          reason: faker.lorem.word(5),
        },
        { correlationId: faker.string.uuid() },
      )
  }
}
