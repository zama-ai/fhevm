import { ValidateAddress } from '#tests/dapp.manager.js'
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
      let validateAddress: ValidateAddress | undefined
      let chainId: string
      let address: string

      beforeEach(async () => {
        chainId = faker.string.numeric(5)
        address = faker.string.hexadecimal({ length: 40 })
        const [result] = await Promise.all([
          manager.dapp.valiateAddress({
            token,
            chainId,
            address,
          }),
          manager.sendMessage(
            JSON.stringify(
              back.addressValidationConfirmed(
                {
                  requestId: faker.string.uuid(),
                  chainId,
                  address,
                },
                { correlationId: faker.string.uuid() },
              ),
            ),
          ),
        ])
        if (result.success) {
          validateAddress = result.data
        }
      })

      test('then it returns true', async () => {
        expect(validateAddress).toBeDefined()
        expect(validateAddress?.check).toBe(true)
      })

      test('then message is null', async () => {
        expect(validateAddress).toBeDefined()
        expect(validateAddress?.message).toBeNull()
      })
    })

    describe('when the address is invalid', { timeout: 30_000 }, () => {
      let validateAddress: ValidateAddress | undefined
      let chainId: string
      let address: string

      beforeEach(async () => {
        chainId = faker.string.numeric(5)
        address = faker.string.hexadecimal({ length: 40 })
        const [result] = await Promise.all([
          manager.dapp.valiateAddress({
            token,
            chainId,
            address,
          }),
          manager.sendMessage(
            JSON.stringify(
              back.addressValidationFailed(
                {
                  requestId: faker.string.uuid(),
                  chainId,
                  address,
                  reason: `${address} not valid`,
                },
                { correlationId: faker.string.uuid() },
              ),
            ),
          ),
        ])
        if (result.success) {
          validateAddress = result.data
        } else {
          console.log(`validateAddress: ${JSON.stringify(result)}`)
          expect(result.success).toBe(true)
        }
      })

      test('then it returns false', async () => {
        expect(validateAddress).toBeDefined()
        expect(validateAddress?.check).toBe(false)
      })

      test('then message contains the reason', async () => {
        expect(validateAddress).toBeDefined()
        expect(validateAddress?.message).toMatch(/not valid/)
      })
    })
  })
})
