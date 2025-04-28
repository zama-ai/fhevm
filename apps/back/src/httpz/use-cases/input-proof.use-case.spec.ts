import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest'
import { InputProof } from './input-proof.use-case.js'
import { PRODUCER } from '#constants.js'
import { AppError, Task, timeoutError } from 'utils'
import { faker } from '@faker-js/faker'
import { IProducer } from '#shared/services/producer.js'
import { back } from 'messages'
import { TestBed } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'
import { ApiKeyAllowsRequest } from '#dapps/use-cases/api-key-allows-request.use-case.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'

describe('InputProof', () => {
  // let module: UnitReference
  let useCase: InputProof
  let task: Task<
    {
      handles: string[]
      signatures: string[]
    },
    AppError
  >

  let producer: Mocked<IProducer>
  let syncService: Mocked<SyncService>
  let apiKeyAllowsRequest: Mocked<ApiKeyAllowsRequest>
  let contractChainId: string
  let contractAddress: string
  let userAddress: string
  let ciphertextWithZkpok: string

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(InputProof).compile()

    useCase = unit

    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>
    syncService = unitRef.get(SYNC_SERVICE) as unknown as Mocked<SyncService>

    contractChainId = faker.string.numeric(5)
    contractAddress = faker.string.hexadecimal({ length: 40 })
    userAddress = faker.string.hexadecimal({ length: 40 })
    ciphertextWithZkpok = faker.string.hexadecimal({
      length: { min: 40, max: 100 },
    })

    apiKeyAllowsRequest = unitRef.get(
      ApiKeyAllowsRequest,
    ) as unknown as Mocked<ApiKeyAllowsRequest>
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
          back.httpzInputProofCompleted(
            {
              requestId,
              handles: [faker.string.hexadecimal({ length: 40 })],
              signatures: [faker.string.hexadecimal({ length: 40 })],
            },
            { correlationId: faker.string.uuid() },
          ),
        ).chain(cb)
      })
      task = useCase.execute(
        {
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithZkpok,
        },
        {},
      )
    })

    test('should complete without errors', async () => {
      // Act
      const p = task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({ type: 'back:httpz:input-proof:requested' }),
      )
      const { requestId } = producer.publish.mock.calls[0][0].payload
      expect(requestId).toBeDefined()

      // Assert
      await expect(p).resolves.not.toThrow()
    })
  })
  describe('errors', () => {
    beforeEach(() => {
      vi.useFakeTimers()

      syncService.waitForResponse.mockImplementation((requestId, cb) => {
        return Task.reject<unknown, AppError>(timeoutError()).chain(cb)
      })
      task = useCase.execute(
        {
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithZkpok,
        },
        {},
      )
    })

    afterEach(() => {
      vi.useRealTimers()
    })

    test('should timeout if no response is sent', async () => {
      const p = task.toPromise()
      vi.runAllTimers()
      await expect(p).rejects.toThrowError(/timeout/i)
    })
  })
})
