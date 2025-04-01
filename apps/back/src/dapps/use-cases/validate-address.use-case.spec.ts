import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest'
import {
  ValidateAddress,
  ValidateAddressOutput,
} from './validate-address.use-case.js'
import { IPubSub, PubSub } from 'utils'
import { back } from 'messages'
import { faker } from '@faker-js/faker'

describe('ValidateAddress', () => {
  let useCase: ValidateAddress
  let pupsub: IPubSub<back.BackEvent>
  beforeEach(() => {
    pupsub = new PubSub()
    useCase = new ValidateAddress(pupsub)
  })
  describe('when address is valid', () => {
    let task: Promise<ValidateAddressOutput>
    let requestId: string
    let chainId: string
    let address: string
    beforeEach(() => {
      requestId = faker.string.uuid()
      chainId = faker.string.numeric(5)
      address = faker.string.hexadecimal({ length: 40 })
      task = useCase.execute({ chainId, address }).toPromise()
    })
    test('should return true', async () => {
      pupsub.publish(
        back.addressValidationConfirmed(
          { requestId, chainId, address },
          { correlationId: faker.string.uuid() },
        ),
      )
      await expect(task).resolves.toEqual({ check: true })
    })
  })
  describe('when address is invalid', () => {
    let task: Promise<ValidateAddressOutput>
    let requestId: string
    let chainId: string
    let address: string
    let reason: string
    beforeEach(() => {
      requestId = faker.string.uuid()
      chainId = faker.string.numeric(5)
      address = faker.string.hexadecimal({ length: 40 })
      reason = faker.lorem.paragraph()
      task = useCase.execute({ chainId, address }).toPromise()
    })
    test('should return false', async () => {
      pupsub.publish(
        back.addressValidationFailed(
          { requestId, chainId, address, reason },
          { correlationId: faker.string.uuid() },
        ),
      )
      await expect(task).resolves.toMatchObject({ check: false })
    })
    test('should return the reason', async () => {
      pupsub.publish(
        back.addressValidationFailed(
          { requestId, chainId, address, reason },
          { correlationId: faker.string.uuid() },
        ),
      )
      await expect(task).resolves.toMatchObject({ message: reason })
    })
  })
  describe('errors', () => {
    beforeEach(() => {
      vi.useFakeTimers()
    })
    afterEach(() => {
      vi.useRealTimers()
    })

    test('should timeout', async () => {
      const task = useCase
        .execute({
          chainId: faker.string.numeric(5),
          address: faker.string.hexadecimal({ length: 40 }),
        })
        .toPromise()
      vi.runAllTimers()
      await expect(task).rejects.toThrowError(/timeout/i)
    })

    test('should reject not valid addresses', async () => {
      const task = useCase
        .execute({
          chainId: faker.string.numeric(5),
          address: faker.string.alphanumeric(40),
        })
        .toPromise()
      await expect(task).rejects.toThrowError(/invalid address/i)
    })
  })
})
