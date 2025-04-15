import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest'
import { InputProof } from './input-proof.use-case.js'
import { PRODUCER, PUBSUB } from '#constants.js'
import { AppError, IPubSub, PubSub, Task } from 'utils'
import { faker } from '@faker-js/faker'
import { IProducer } from '#shared/services/producer.js'
import { back } from 'messages'
import { TestBed } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'
import { ApiKeyAllowsRequest } from '#dapps/use-cases/api-key-allows-request.use-case.js'

describe('InputProof', () => {
  // let module: UnitReference
  let useCase: InputProof

  let producer: Mocked<IProducer>
  let pubsub: IPubSub<back.BackEvent>
  let apiKeyAllowsRequest: Mocked<ApiKeyAllowsRequest>
  let contractChainId
  let contractAddress
  let userAddress
  let ciphertextWithZkpok
  let task: Task<
    {
      handles: string[]
      signatures: string[]
    },
    AppError
  >

  beforeEach(async () => {
    pubsub = new PubSub()
    const { unit, unitRef } = await TestBed.solitary(InputProof)
      .mock(PUBSUB)
      .final(pubsub)
      .compile()

    useCase = unit

    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>

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

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('when input proof is valid', () => {
    test('should complete without errors', async () => {
      // Act
      const p = task.toPromise()
      expect(producer.publish).toHaveBeenCalledExactlyOnceWith(
        expect.objectContaining({ type: 'back:httpz:input-proof:requested' }),
      )
      const { requestId } = producer.publish.mock.calls[0][0].payload
      expect(requestId).toBeDefined()

      await pubsub
        .publish(
          back.httpzInputProofCompleted(
            {
              requestId,
              handles: [faker.string.hexadecimal({ length: 40 })],
              signatures: [faker.string.hexadecimal({ length: 40 })],
            },
            { correlationId: faker.string.uuid() },
          ),
        )
        .toPromise()

      // Assert
      await expect(p).resolves.not.toThrow()
    })
  })
  describe('errors', () => {
    beforeEach(() => {
      vi.useFakeTimers()
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
