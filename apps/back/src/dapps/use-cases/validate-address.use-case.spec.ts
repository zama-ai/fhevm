import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest'
import {
  ValidateAddress,
  ValidateAddressOutput,
} from './validate-address.use-case.js'
import { AppError, Task, timeoutError } from 'utils'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { TestBed } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { PRODUCER } from '#constants.js'
import { IProducer } from '#shared/services/producer.js'

describe('ValidateAddress', () => {
  let useCase: ValidateAddress
  let syncService: Mocked<SyncService>
  let producer: Mocked<IProducer>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(ValidateAddress).compile()

    syncService = unitRef.get(SYNC_SERVICE) as unknown as Mocked<SyncService>
    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>
    useCase = unit
  })

  describe('when address is valid', () => {
    let task: Promise<ValidateAddressOutput>
    // let requestId: string
    let chainId: string
    let address: string
    beforeEach(() => {
      // requestId = faker.string.uuid()
      chainId = faker.string.numeric(5)
      address = faker.string.hexadecimal({ length: 40 })
      syncService.waitForResponse.mockImplementation((requestId, cb) => {
        return Task.of<unknown, AppError>(
          back.addressValidationConfirmed(
            { requestId, chainId, address },
            { correlationId: faker.string.uuid() },
          ),
        ).chain(cb)
      })
      producer.publish.mockReturnValue(Task.of(void 0))
      task = useCase.execute({ chainId, address }).toPromise()
    })
    test('should return true', async () => {
      await expect(task).resolves.toEqual({ check: true })
    })
  })

  describe('when address is invalid', () => {
    let task: Promise<ValidateAddressOutput>
    let chainId: string
    let address: string
    let reason: string

    beforeEach(() => {
      chainId = faker.string.numeric(5)
      address = faker.string.hexadecimal({ length: 40 })
      reason = faker.lorem.paragraph()
      syncService.waitForResponse.mockImplementation((requestId, cb) => {
        return Task.of<unknown, AppError>(
          back.addressValidationFailed(
            { requestId, chainId, address, reason },
            { correlationId: faker.string.uuid() },
          ),
        ).chain(cb)
      })
      producer.publish.mockReturnValue(Task.of(void 0))
      task = useCase.execute({ chainId, address }).toPromise()
    })

    test('should return false', async () => {
      await expect(task).resolves.toMatchObject({ check: false })
    })

    test('should return the reason', async () => {
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
      producer.publish.mockReturnValue(Task.of(void 0))
      syncService.waitForResponse.mockImplementation((requestId, cb) => {
        return Task.reject<unknown, AppError>(timeoutError()).chain(cb)
      })
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
      await expect(task).rejects.toThrowError(/address should be/i)
    })
  })
})
