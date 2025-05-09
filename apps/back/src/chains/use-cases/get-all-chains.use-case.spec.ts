import { beforeEach, describe, expect, test } from 'vitest'
import { GetAllChains } from './get-all-chains.use-case.js'
import { TestBed } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'
import {
  CHAINS_REPOSITORY,
  ChainsRepository,
} from '#chains/domain/repositories/chains.repository.js'
import { Task, unknownError } from 'utils'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import { faker } from '@faker-js/faker'
import { Chain } from '#chains/domain/entities/chain.js'

describe('GetAllChains', () => {
  let useCase: GetAllChains
  let repo: Mocked<ChainsRepository>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(GetAllChains).compile()

    useCase = unit
    repo = unitRef.get(CHAINS_REPOSITORY) as unknown as Mocked<ChainsRepository>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given no chain exists', () => {
    beforeEach(() => {
      repo.getChains.mockReturnValue(Task.of([]))
    })

    test('when executing, then it should return an empty array', async () => {
      const result = await useCase.execute().toPromise()
      expect(result).toEqual([])
    })
  })

  describe('given chains exist', () => {
    let chains: Chain[]

    beforeEach(() => {
      chains = [
        Chain.parse({
          id: ChainId.from(faker.number.int({ min: 1, max: 100_000 })).unwrap()
            .value,
          name: faker.string.alphanumeric(10),
          description: faker.lorem.words(5),
        }).unwrap(),
        Chain.parse({
          id: ChainId.from(faker.number.int({ min: 1, max: 100_000 })).unwrap()
            .value,
          name: faker.string.alphanumeric(10),
          description: null,
        }).unwrap(),
        Chain.parse({
          id: ChainId.from(faker.number.int({ min: 1, max: 100_000 })).unwrap()
            .value,
          name: faker.string.alphanumeric(10),
          description: undefined,
        }).unwrap(),
      ]

      repo.getChains.mockReturnValue(Task.of(chains))
    })

    test('when executing, then it should return an array of chains', async () => {
      const result = await useCase.execute().toPromise()
      expect(result.length).toEqual(chains.length)
      result.forEach((chain, index) => {
        expect(chain.id.value).toEqual(chains[index].id.value)
        expect(chain.name).toEqual(chains[index].name)
        expect(chain.description).toEqual(chains[index].description)
      })
    })
  })

  describe('given an error occurs', () => {
    let message: string
    beforeEach(() => {
      message = faker.lorem.words(5)
      repo.getChains.mockReturnValue(Task.reject(unknownError(message)))
    })

    test('when executing, then it should return an error', async () => {
      await expect(useCase.execute().toPromise()).rejects.toThrowError(message)
    })
  })
})
