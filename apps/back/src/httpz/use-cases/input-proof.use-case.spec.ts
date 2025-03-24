import { Test, TestingModule } from '@nestjs/testing'
import { afterEach, beforeEach, describe, expect, test, vi } from 'vitest'
import { InputProof } from './input-proof.use-case.js'
import { PRODUCER, PUBSUB } from '#constants.js'
import { AppError, IPubSub, PubSub, Task } from 'utils'
import { mock, MockProxy } from 'vitest-mock-extended'
import { faker } from '@faker-js/faker'
import { IProducer } from '#shared/services/producer.js'
import { back } from 'messages'

describe('InputProof', () => {
  let module: TestingModule
  let useCase: InputProof

  let producer: MockProxy<IProducer>
  let pubsub: IPubSub<back.BackEvent>
  let contractChainId
  let contractAddress
  let userAddress
  let ciphertextWithZkpok
  let task: Task<void, AppError>

  beforeEach(async () => {
    module = await Test.createTestingModule({
      providers: [
        InputProof,
        {
          provide: PUBSUB,
          useValue: new PubSub(),
        },
        {
          provide: PRODUCER,
          useValue: mock(),
        },
      ],
    }).compile()

    useCase = module.get(InputProof)

    producer = module.get(PRODUCER)
    pubsub = module.get(PUBSUB)
    contractChainId = faker.string.numeric(5)
    contractAddress = faker.string.hexadecimal({ length: 40 })
    userAddress = faker.string.hexadecimal({ length: 40 })
    ciphertextWithZkpok = faker.string.hexadecimal({
      length: { min: 40, max: 100 },
    })

    producer.publish.mockReturnValue(Task.of(void 0))
    task = useCase.execute({
      contractChainId,
      contractAddress,
      userAddress,
      ciphertextWithZkpok,
    })
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
            { requestId, success: true },
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
