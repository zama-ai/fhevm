import { beforeEach, describe, expect, test } from 'vitest'
import {
  ValidateAddress,
  ValidateAddressOutput,
  ValidateAddressWithSync,
} from './validate-address.use-case.js'
import { AppError, Task, timeoutError, unknownError } from 'utils'
import { back } from 'messages'
import { faker } from '@faker-js/faker'
import { TestBed } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { PRODUCER } from '#constants.js'
import { IProducer } from '#shared/services/producer.js'
import { SyncInstances } from '#shared/use-cases/sync-instances.use-case.js'

describe('ValidateAddress', () => {
  let useCase: ValidateAddress
  let producer: Mocked<IProducer>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(ValidateAddress).compile()

    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>
    useCase = unit
  })

  test('it should publish the message', async () => {
    const chainId = faker.number.int({ min: 1, max: 100_000 })
    const address = faker.string.hexadecimal({ length: 40 })
    producer.publish.mockReturnValue(Task.of(void 0))

    await useCase.execute({ chainId, address }).toPromise()

    expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
      expect.objectContaining({
        type: 'back:address:validation:requested',
        payload: expect.objectContaining({
          chainId,
          address,
          requestId: expect.any(String),
        }),
        meta: { correlationId: expect.any(String) },
      }),
    )
  })

  describe('when producer fails', () => {
    let message: string
    beforeEach(() => {
      message = faker.string.alphanumeric({ length: { min: 10, max: 100 } })
      producer.publish.mockReturnValue(Task.reject(unknownError(message)))
    })

    test('then it should forward the error', async () => {
      await expect(
        useCase
          .execute({
            chainId: faker.number.int({ min: 1, max: 100_000 }),
            address: faker.string.hexadecimal({ length: 40 }),
          })
          .toPromise(),
      ).rejects.toThrowError(message)
    })
  })
})

describe('ValidateAddressWithSync', () => {
  let useCase: ValidateAddressWithSync
  let validateAddress: Mocked<ValidateAddress>
  let syncService: Mocked<SyncService>
  let syncInstances: Mocked<SyncInstances>

  let chainId: number
  let address: string

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      ValidateAddressWithSync,
    ).compile()

    useCase = unit
    validateAddress = unitRef.get(
      ValidateAddress,
    ) as unknown as Mocked<ValidateAddress>
    syncService = unitRef.get(SYNC_SERVICE) as unknown as Mocked<SyncService>
    syncInstances = unitRef.get(
      SyncInstances,
    ) as unknown as Mocked<SyncInstances>
    syncInstances.listenToEvent.mockReturnValue(void 0)

    chainId = faker.number.int({ min: 1, max: 100_000 })
    address = faker.string.hexadecimal({ length: 40 })
  })

  test('should be defined', () => {
    expect(ValidateAddressWithSync).toBeDefined()
  })

  test('should call validateAddress', async () => {
    const requestId = faker.string.uuid()
    validateAddress.execute.mockReturnValue(Task.of({ check: true }))
    syncService.waitForResponse.mockImplementation((requestId, cb) => {
      return Task.of<unknown, AppError>(
        back.addressValidationConfirmed(
          { requestId, chainId, address },
          { correlationId: faker.string.uuid() },
        ),
      ).chain(cb)
    })
    await useCase.execute({ chainId, address }, { requestId }).toPromise()
    expect(validateAddress.execute).toHaveBeenCalledExactlyOnceWith(
      {
        chainId,
        address,
      },
      { requestId },
    )
  })

  describe('when synchronization completes', () => {
    describe('and the address is valid', () => {
      let chainId: number
      let address: string
      beforeEach(() => {
        chainId = faker.number.int({ min: 1, max: 100_000 })
        address = faker.string.hexadecimal({ length: 40 })
        validateAddress.execute.mockReturnValue(Task.of({ check: true }))
        syncService.waitForResponse.mockImplementation((requestId, cb) => {
          return Task.of<unknown, AppError>(
            back.addressValidationConfirmed(
              { requestId, chainId, address },
              { correlationId: faker.string.uuid() },
            ),
          ).chain(cb)
        })
      })
      test('then it should return true', async () => {
        await expect(
          useCase.execute({ chainId, address }).toPromise(),
        ).resolves.toEqual({
          check: true,
        })
      })
    })

    describe('and the address is invalid', () => {
      let chainId: number
      let address: string
      let reason: string

      beforeEach(() => {
        chainId = faker.number.int({ min: 1, max: 100_000 })
        address = faker.string.hexadecimal({ length: 40 })
        reason = faker.lorem.paragraph()
        validateAddress.execute.mockReturnValue(Task.of({ check: true }))
        syncService.waitForResponse.mockImplementation((requestId, cb) => {
          return Task.of<unknown, AppError>(
            back.addressValidationFailed(
              { requestId, chainId, address, reason },
              { correlationId: faker.string.uuid() },
            ),
          ).chain(cb)
        })
      })

      test('should return false', async () => {
        await expect(
          useCase.execute({ chainId, address }).toPromise(),
        ).resolves.toMatchObject({ check: false })
      })

      test('should return the reason', async () => {
        await expect(
          useCase.execute({ chainId, address }).toPromise(),
        ).resolves.toMatchObject({ message: reason })
      })
    })
  })

  describe('when synchronization times out', () => {
    beforeEach(() => {
      validateAddress.execute.mockReturnValue(Task.of({ check: true }))
      syncService.waitForResponse.mockImplementation((requestId, cb) => {
        return Task.reject<unknown, AppError>(timeoutError()).chain(cb)
      })
    })

    test('then it should fail', async () => {
      await expect(
        useCase.execute({ chainId, address }).toPromise(),
      ).rejects.toThrowError(/timeout/i)
    })
  })

  describe('when address validation fails', () => {
    let message: string
    beforeEach(() => {
      message = faker.string.alphanumeric({ length: { min: 10, max: 100 } })
      validateAddress.execute.mockReturnValue(
        Task.reject<ValidateAddressOutput, AppError>(unknownError(message)),
      )
    })

    test('then it should fail', async () => {
      await expect(
        useCase.execute({ chainId, address }).toPromise(),
      ).rejects.toThrowError(message)
    })
  })
})
