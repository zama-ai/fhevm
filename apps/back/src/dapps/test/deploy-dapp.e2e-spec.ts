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
  }, 30_000)

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
          expect(result.errors[0].message).contain(
            `missing dApp's chainId and address`,
          )
        }
      })
    })

    describe('when deploying a dapp after updating the dapp address', () => {
      let status: DAppStatus

      beforeEach(async () => {
        const chainId = 11155111 // Sepolia

        const updateDApp = await manager.dapp.updateDApp({
          token,
          dappId,
          chainId,
          address: faker.string.hexadecimal({ length: 40 }),
        })
        if (!updateDApp.success) {
          console.log(`updateDApp: ${JSON.stringify(updateDApp)}`)
          expect(updateDApp.success, 'Failed to update dApp address').toBe(true)
        }
        const deployDapp = await manager.dapp.deployDApp({
          token,
          dappId,
        })
        if (!deployDapp.success) {
          console.log(`deployDapp: ${JSON.stringify(deployDapp)}`)
          expect(deployDapp.success).toBe(true)
        } else {
          status = deployDapp.data.status
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
    // NOTE: I need to use faker here because I'm using `describe.each` later
    const chainId = 11155111 // Sepolia

    beforeEach(async () => {
      const login = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      if (login.success) {
        token = login.data.token
        teamId = login.data.user.teams[0].id

        const createDapp = await manager.dapp.createDApp({
          token,
          teamId,
          name: faker.string.alphanumeric(10),
          chainId,
          address: faker.string.hexadecimal({ length: 40 }),
        })
        if (createDapp.success) {
          dappId = createDapp.data.id
          const deployDapp = await manager.dapp.deployDApp({
            token,
            dappId,
          })
          if (!deployDapp.success) {
            console.log(`deployDapp: ${JSON.stringify(deployDapp)}`)
            expect(deployDapp.success).toBe(true)
          }
        } else {
          console.log(`createDapp: ${JSON.stringify(createDapp)}`)
          expect(createDapp.success).toBe(true)
        }
      } else {
        console.log(`failed to login: ${JSON.stringify(login)}`)
        expect(login.success, 'Failed to login the user').toBe(true)
      }
    })

    describe.each([
      {
        event: back.dappValidationRequested(
          {
            requestId: faker.string.uuid(),
            dAppId: '',
            chainId,
            address: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
        status: 'DEPLOYING',
      },
      {
        event: back.dappValidationConfirmed(
          {
            requestId: faker.string.uuid(),
            dAppId: '',
            owner: faker.string.hexadecimal({ length: 40 }),
          },
          { correlationId: faker.string.uuid() },
        ),
        status: 'LIVE',
      },
      {
        event: back.dappValidationFailed(
          {
            requestId: faker.string.uuid(),
            dAppId: '',
            reason: faker.lorem.word(5),
          },
          { correlationId: faker.string.uuid() },
        ),
        status: 'FAILED',
      },
    ] satisfies { event: back.BackEvent; status: DAppStatus }[])(
      'when receiving `$event.type` event',
      ({ event, status }) => {
        beforeEach(async () => {
          // NOTE: I need to override the dAppId with the created one
          // because it doesn't exist at test description step.
          event.payload.dAppId = dappId
          await manager.sendMessage(JSON.stringify(event))
        })

        test(`then the dapp status should be "${status}"`, async () => {
          await vi.waitUntil(async () => {
            const size = await manager.getBackQueueSize()
            return size === 0
          })

          const result = await manager.dapp.getDapp({
            token,
            dappId,
          })
          if (result.success) {
            expect(result.data.status).toBe(status)
          } else {
            console.log(result)
            expect(result.success).toBe(true)
          }
        })
      },
    )
  })
})
