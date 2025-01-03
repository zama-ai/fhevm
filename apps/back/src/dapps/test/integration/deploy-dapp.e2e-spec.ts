import {
  DeployDappResult,
  GraphQlResponse,
  IntegrationManager,
} from '@/tests/integration.manager'
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
import { DAppStatus } from '@/dapps/domain/entities/dapp'
import {
  AppDeploymentMessage,
  completed,
  failed,
  isAppDeploymentMessage,
  requested,
} from 'messages'

describe('deploy-dapp', () => {
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
        expect(dappResult.success).toBe(true)
        if (dappResult.success) {
          dappId = dappResult.data.dapp.id
        }
      }
    })

    describe('when deploying a dapp', () => {
      let result: GraphQlResponse<{ dapp: DeployDappResult }>

      beforeEach(async () => {
        result = await manager.dapp.deployDApp({
          token,
          dappId,
        })
      })

      test('then it fails due missing address', () => {
        expect(result.success).toBe(false)
        if (!result.success) {
          expect(result.errors).toBeDefined()
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
          status = result.data.dapp.status
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
          dappId = dappResult.data.dapp.id
          const result = await manager.dapp.deployDApp({
            token,
            dappId,
          })
          expect(result.success).toBe(true)
        }
      }
    })

    describe.each([
      'app-deployment.requested',
      'app-deployment.completed',
      'app-deployment.failed',
    ] satisfies AppDeploymentMessage['type'][])(
      'when receiving `%s` event',
      type => {
        let status: DAppStatus
        switch (type) {
          case 'app-deployment.completed':
            status = 'LIVE'
            break
          case 'app-deployment.failed':
            status = 'DRAFT'
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
          await vi.waitFor(async () => {
            const size = await manager.getQueueSize()
            expect(size).toBe(0)
            return
          })

          const result = await manager.dapp.getDapp({
            token,
            dappId,
          })

          expect(result.success).toBe(true)
          if (result.success) {
            expect(result.data.status).toBe(status)
          }
        })
      },
    )
  })
})

function genMessage(
  type: AppDeploymentMessage['type'],
  dappId: string,
): AppDeploymentMessage | undefined {
  switch (type) {
    case 'app-deployment.requested':
      return requested({
        applicationId: dappId,
        deploymentId: faker.string.uuid(),
        address: faker.string.hexadecimal({ length: 40 }),
        chainId: '1',
      })
    case 'app-deployment.completed':
      return completed({
        applicationId: dappId,
        deploymentId: faker.string.uuid(),
      })
    case 'app-deployment.failed':
      return failed({
        applicationId: dappId,
        deploymentId: faker.string.uuid(),
      })
  }
}
