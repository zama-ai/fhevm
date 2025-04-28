import { IntegrationManager } from '#tests/integration.manager.js'
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

describe('validate-address', () => {
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

  describe('given a logged user validate an address', () => {
    let token: string

    beforeEach(async () => {
      const result = await manager.auth.login(
        { email: faker.internet.email(), password: faker.internet.password() },
        { signup: true },
      )
      if (result.success) {
        token = result.data.token
      }
    })

    describe('when the address is valid', () => {
      test('then it returns true', async () => {
        const validateAddress = await sendValidateAddressRequest(
          manager,
          token,
          true,
        )
        expect(validateAddress).toBeDefined()
        expect(validateAddress?.check).toBe(true)
      })

      test('then message is null', async () => {
        const validateAddress = await sendValidateAddressRequest(
          manager,
          token,
          true,
        )
        expect(validateAddress).toBeDefined()
        expect(validateAddress?.message).toBeNull()
      })
    })

    describe('when the address is invalid', { timeout: 30_000 }, () => {
      test('then it returns false', async () => {
        const validateAddress = await sendValidateAddressRequest(
          manager,
          token,
          false,
        )
        expect(validateAddress).toBeDefined()
        expect(validateAddress?.check).toBe(false)
      })

      test('then message contains the reason', async () => {
        const validateAddress = await sendValidateAddressRequest(
          manager,
          token,
          false,
        )
        expect(validateAddress).toBeDefined()
        expect(validateAddress?.message).toMatch(/not valid/)
      })
    })
  })
})

async function sendValidateAddressRequest(
  manager: IntegrationManager,
  token: string,
  success: boolean,
) {
  const chainId = faker.string.numeric(5)
  const address = faker.string.hexadecimal({ length: 40 })

  const promise = manager.dapp.valiateAddress({
    token,
    chainId,
    address,
  })

  await vi.waitUntil(async () => {
    const size = await manager.getOrchQueueSize()
    return size > 0
  })
  const message = await manager.getMessageFromOrchQueue()
  const event = JSON.parse(message!)
  expect(back.isBackEvent(event)).toBe(true)
  expect(event.type).toBe('back:address:validation:requested')
  expect(event.payload).toEqual({
    chainId,
    address,
    requestId: event.payload.requestId,
  })
  manager.sendMessage(
    success
      ? back.addressValidationConfirmed(
          {
            requestId: event.payload.requestId,
            chainId,
            address,
          },
          { correlationId: faker.string.uuid() },
        )
      : back.addressValidationFailed(
          {
            requestId: event.payload.requestId,
            chainId,
            address,
            reason: 'not valid',
          },
          { correlationId: faker.string.uuid() },
        ),
  )

  const result = await promise
  if (result.success) {
    return result.data
  }
  console.log(`validateAddress failed: ${JSON.stringify(result)}`)
  expect(result.success).toBe(true)
  return null
}
