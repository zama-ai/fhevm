import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { GetChainById } from './get-chain-by-id.use-case.js'
import { TestBed } from '@suites/unit'
import {
  CHAINS_REPOSITORY,
  ChainsRepository,
} from '#chains/domain/repositories/chains.repository.js'
import { notFoundError, Task } from 'utils'
import { faker } from '@faker-js/faker'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import { Chain } from '#chains/domain/entities/chain.js'

describe('GetChainById', () => {
  let useCase: GetChainById
  let repo: Mocked<ChainsRepository>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(GetChainById).compile()

    useCase = unit
    repo = unitRef.get(CHAINS_REPOSITORY) as unknown as Mocked<ChainsRepository>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given no chain exists', () => {
    let chainId: number

    beforeEach(() => {
      chainId = faker.number.int({ min: 1, max: 100_000 })
      return repo.getChainById.mockReturnValue(Task.reject(notFoundError()))
    })

    test('when executing, then it should return a not found error', async () => {
      await expect(
        useCase.execute({ id: chainId }).toPromise(),
      ).rejects.toThrowError(/not found/i)
      expect(repo.getChainById).toHaveBeenCalledWith(
        ChainId.from(chainId).unwrap(),
      )
    })
  })

  describe('given a chain exists', () => {
    let chainId: number
    let chain: Chain

    beforeEach(() => {
      chainId = faker.number.int({ min: 1, max: 100_000 })
      chain = Chain.parse({
        id: chainId,
        name: faker.string.alphanumeric(10),
        description: faker.lorem.words(5),
      }).unwrap()

      repo.getChainById.mockReturnValue(Task.of(chain))
    })

    test('when executing, then it should return the chain', async () => {
      const result = await useCase.execute({ id: chainId }).toPromise()
      expect(result).toEqual(chain)
      expect(repo.getChainById).toHaveBeenCalledWith(
        ChainId.from(chainId).unwrap(),
      )
    })
  })
})
