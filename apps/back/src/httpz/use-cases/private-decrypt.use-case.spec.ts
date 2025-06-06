import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest'
import { PrivateDecrypt } from './private-decrypt.use-case.js'
import { PRODUCER } from '#constants.js'
import { AppError, Task, timeoutError } from 'utils'
import { faker } from '@faker-js/faker'
import { IProducer } from '#shared/services/producer.js'
import { back } from 'messages'
import { TestBed } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'
import {
  type IApiKeyAllowsRequest,
  API_KEY_ALLOWS_REQUEST,
} from '#dapps/use-cases/api-key-allows-request.use-case.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'

describe('PrivateDecrypt', () => {
  // let module: UnitReference
  let useCase: PrivateDecrypt
  let task: ReturnType<PrivateDecrypt['execute']>

  let producer: Mocked<IProducer>
  let syncService: Mocked<SyncService>
  let apiKeyAllowsRequest: Mocked<IApiKeyAllowsRequest>
  let contractChainId: string
  let contractAddress: string
  let userAddress: string
  let handle: string

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(PrivateDecrypt).compile()

    useCase = unit

    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>
    syncService = unitRef.get(SYNC_SERVICE) as unknown as Mocked<SyncService>

    contractChainId = faker.string.numeric(5)
    contractAddress = faker.string.hexadecimal({ length: 40 })
    userAddress = faker.string.hexadecimal({ length: 40 })
    handle = faker.string.hexadecimal({
      length: 40,
    })

    apiKeyAllowsRequest = unitRef.get(
      API_KEY_ALLOWS_REQUEST,
    ) as unknown as Mocked<IApiKeyAllowsRequest>
    apiKeyAllowsRequest.execute.mockReturnValue(Task.of(void 0))

    producer.publish.mockReturnValue(Task.of(void 0))
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('when input proof is valid', () => {
    beforeEach(() => {
      syncService.waitForResponse.mockImplementation((requestId, cb) => {
        return Task.of<unknown, AppError>(
          back.httpzPrivateDecryptCompleted(
            {
              requestId,
              gatewayRequestId: faker.number.int({ min: 0, max: 213 }),
              decryptedValue: faker.string.hexadecimal({ length: 40 }),
              signatures: [faker.string.hexadecimal({ length: 40 })],
            },
            { correlationId: faker.string.uuid() },
          ),
        ).chain(cb)
      })

      task = useCase.execute(
        {
          contractsChainId: contractChainId,
          requestValidity: {
            startTimestamp: faker.string.numeric(10),
            durationDays: faker.string.numeric(2),
          },
          signature: faker.string.hexadecimal({ length: 40 }),
          publicKey: faker.string.hexadecimal({ length: 40 }),
          contractsAddresses: [contractAddress],
          userAddress,
          handleContractPairs: [
            {
              handle,
              contractAddress: contractAddress,
            },
          ],
        },
        {},
      )
    })

    test('should complete without errors', async () => {
      // Act
      const p = task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({
          type: 'back:httpz:private-decrypt:requested',
        }),
      )
      const { requestId } = producer.publish.mock.calls[0][0].payload
      expect(requestId).toBeDefined()

      // Assert
      await expect(p).resolves.not.toThrow()
    })
  })

  // describe('errors', () => {
  //   beforeEach(() => {
  //     vi.useFakeTimers()
  //
  //     syncService.waitForResponse.mockImplementation((requestId, cb) => {
  //       return Task.reject<unknown, AppError>(timeoutError()).chain(cb)
  //     })
  //     task = useCase.execute(
  //       {
  //         contractChainId,
  //         contractAddress,
  //         userAddress,
  //         ciphertextWithInputVerification,
  //       },
  //       {},
  //     )
  //   })
  //
  //   afterEach(() => {
  //     vi.useRealTimers()
  //   })
  //
  //   test('should timeout if no response is sent', async () => {
  //     const p = task.toPromise()
  //     vi.runAllTimers()
  //     await expect(p).rejects.toThrowError(/timeout/i)
  //   })
  // })
})
