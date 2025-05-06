import { beforeEach, describe, expect, Mock, test } from 'vitest'
import {
  IInputProof,
  InputProof,
  InputProofWithApiKey,
  InputProofWithSync,
} from './input-proof.use-case.js'
import { PRODUCER } from '#constants.js'
import {
  AppError,
  Task,
  timeoutError,
  unauthorizedError,
  unknownError,
} from 'utils'
import { faker } from '@faker-js/faker'
import { IProducer } from '#shared/services/producer.js'
import { back } from 'messages'
import { TestBed } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'
import {
  API_KEY_ALLOWS_REQUEST,
  type IApiKeyAllowsRequest,
} from '#dapps/use-cases/api-key-allows-request.use-case.js'
import { SYNC_SERVICE, SyncService } from '#shared/services/sync.service.js'
import { SyncInstances } from '#shared/use-cases/sync-instances.use-case.js'

describe('InputProof', () => {
  let useCase: InputProof
  let task: ReturnType<InputProof['execute']>

  let producer: Mocked<IProducer>
  let requestId: string
  let contractChainId: string
  let contractAddress: string
  let userAddress: string
  let ciphertextWithInputVerification: string

  beforeEach(async () => {
    requestId = faker.string.uuid()

    const { unit, unitRef } = await TestBed.solitary(InputProof).compile()

    useCase = unit

    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>

    contractChainId = faker.string.numeric(5)
    contractAddress = faker.string.hexadecimal({ length: 40 })
    userAddress = faker.string.hexadecimal({ length: 40 })
    ciphertextWithInputVerification = faker.string.hexadecimal({
      length: { min: 40, max: 100 },
    })

    producer.publish.mockReturnValue(Task.of(void 0))
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('when input proof is valid', () => {
    beforeEach(() => {
      task = useCase.execute(
        {
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithInputVerification,
        },
        { requestId },
      )
    })

    test('should complete without errors', async () => {
      // Act
      const p = task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({ type: 'back:httpz:input-proof:requested' }),
      )
      const { requestId: requestIdFromEvent } =
        producer.publish.mock.calls[0][0].payload
      expect(requestIdFromEvent).toBeDefined()
      expect(requestIdFromEvent).toEqual(requestId)

      // Assert
      await expect(p).resolves.not.toThrow()
    })
  })

  describe('errors', () => {
    let message: string
    beforeEach(() => {
      message = faker.string.alphanumeric({ length: { min: 10, max: 100 } })
      producer.publish.mockReturnValue(Task.reject(unknownError(message)))
    })

    test('should forward error', async () => {
      await expect(
        useCase
          .execute(
            {
              contractChainId,
              contractAddress,
              userAddress,
              ciphertextWithInputVerification,
            },
            {
              requestId,
            },
          )
          .toPromise(),
      ).rejects.toThrowError(message)
    })
  })
})

describe('InputProofWithSync', () => {
  let useCase: InputProofWithSync
  let inputProof: Mocked<InputProof>
  let syncService: Mocked<SyncService>
  let syncInstances: Mocked<SyncInstances>

  let contractChainId: string
  let contractAddress: string
  let userAddress: string
  let ciphertextWithInputVerification: string

  beforeEach(async () => {
    const { unit, unitRef } =
      await TestBed.solitary(InputProofWithSync).compile()

    useCase = unit
    inputProof = unitRef.get(InputProof) as unknown as Mocked<InputProof>
    syncService = unitRef.get(SYNC_SERVICE) as unknown as Mocked<SyncService>
    syncInstances = unitRef.get(
      SyncInstances,
    ) as unknown as Mocked<SyncInstances>
    syncInstances.listenToEvent.mockReturnValue(void 0)

    contractChainId = faker.string.numeric(5)
    contractAddress = faker.string.hexadecimal({ length: 40 })
    userAddress = faker.string.hexadecimal({ length: 40 })
    ciphertextWithInputVerification = faker.string.hexadecimal({
      length: { min: 40, max: 100 },
    })
  })

  test('should be defined', () => {
    expect(InputProofWithSync).toBeDefined()
  })

  describe('when synchronization completes', () => {
    let handles: string[]
    let signatures: string[]

    beforeEach(() => {
      handles = [faker.string.hexadecimal({ length: 40 })]
      signatures = [faker.string.hexadecimal({ length: 40 })]

      inputProof.execute.mockReturnValue(
        Task.of({ handles: [], signatures: [] }),
      )
      syncService.waitForResponse.mockImplementation((requestId, cb) => {
        return Task.of<back.BackEvent, AppError>(
          back.httpzInputProofCompleted(
            {
              requestId,
              handles,
              signatures,
            },
            { correlationId: faker.string.uuid() },
          ),
        ).chain(cb)
      })
    })

    test('then it should returns the right result', async () => {
      const result = await useCase
        .execute({
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithInputVerification,
        })
        .toPromise()
      expect(result).toEqual({ handles, signatures })
    })
  })

  describe('when synchronization times out', () => {
    beforeEach(() => {
      inputProof.execute.mockReturnValue(
        Task.of({ handles: [], signatures: [] }),
      )
      syncService.waitForResponse.mockImplementation((requestId, cb) => {
        return Task.reject<unknown, AppError>(timeoutError()).chain(cb)
      })
    })

    test('then it should fail', async () => {
      await expect(
        useCase
          .execute({
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithInputVerification,
          })
          .toPromise(),
      ).rejects.toThrowError(/timeout/i)
    })
  })

  describe('when input proof fails', () => {
    let message: string
    beforeEach(() => {
      message = faker.string.alphanumeric({ length: { min: 10, max: 100 } })
      inputProof.execute.mockReturnValue(
        Task.reject<{ handles: string[]; signatures: string[] }, AppError>(
          unknownError(message),
        ),
      )
    })

    test('then it should forward the error', async () => {
      await expect(
        useCase
          .execute({
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithInputVerification,
          })
          .toPromise(),
      ).rejects.toThrowError(message)
    })
  })
})

describe('InputProofWithApiKey', () => {
  let useCase: InputProofWithApiKey
  let inputProof: Mocked<InputProofWithSync>
  let apiKeyAllowsRequest: Mocked<IApiKeyAllowsRequest>

  let contractChainId: string
  let contractAddress: string
  let userAddress: string
  let ciphertextWithInputVerification: string

  beforeEach(async () => {
    const { unit, unitRef } =
      await TestBed.solitary(InputProofWithApiKey).compile()

    useCase = unit

    inputProof = unitRef.get(
      InputProofWithSync,
    ) as unknown as Mocked<InputProofWithSync>
    apiKeyAllowsRequest = unitRef.get(
      API_KEY_ALLOWS_REQUEST,
    ) as unknown as Mocked<IApiKeyAllowsRequest>

    contractChainId = faker.string.numeric(5)
    contractAddress = faker.string.hexadecimal({ length: 40 })
    userAddress = faker.string.hexadecimal({ length: 40 })
    ciphertextWithInputVerification = faker.string.hexadecimal({
      length: { min: 40, max: 100 },
    })
  })

  test('should be defined', () => {
    expect(InputProofWithApiKey).toBeDefined()
  })

  describe('when request is allowed', () => {
    let handles: string[]
    let signatures: string[]

    beforeEach(() => {
      handles = [faker.string.hexadecimal({ length: 40 })]
      signatures = [faker.string.hexadecimal({ length: 40 })]
        ; (
          apiKeyAllowsRequest.execute as Mock<IApiKeyAllowsRequest['execute']>
        ).mockReturnValue(Task.of(void 0))
        ; (inputProof.execute as Mock<IInputProof['execute']>).mockReturnValue(
          Task.of({ handles, signatures }),
        )
    })
    test('should call the input proof use case', async () => {
      await useCase
        .execute(
          {
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithInputVerification,
          },
          {},
        )
        .toPromise()
      expect(inputProof.execute).toHaveBeenCalledExactlyOnceWith(
        {
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithInputVerification,
        },
        expect.anything(),
      )
    })

    test('should return the right handles and signatures', async () => {
      const result = await useCase
        .execute({
          contractChainId,
          contractAddress,
          userAddress,
          ciphertextWithInputVerification,
        })
        .toPromise()
      expect(result).toEqual({ handles, signatures })
    })
  })

  describe('when request is not allowed', () => {
    beforeEach(() => {
      ; (
        apiKeyAllowsRequest.execute as Mock<IApiKeyAllowsRequest['execute']>
      ).mockReturnValue(Task.reject(unauthorizedError()))
    })

    test('should fail with an unauthorized error', async () => {
      await expect(
        useCase
          .execute({
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithInputVerification,
          })
          .toPromise(),
      ).rejects.toThrowError(/unauthorized/i)
    })

    test('should not call the input proof use case', async () => {
      try {
        await useCase
          .execute({
            contractChainId,
            contractAddress,
            userAddress,
            ciphertextWithInputVerification,
          })
          .toPromise()
        expect.fail('should have failed')
      } catch {
        /* ignored */
      }
      expect(inputProof.execute).not.toHaveBeenCalled()
    })
  })
})
