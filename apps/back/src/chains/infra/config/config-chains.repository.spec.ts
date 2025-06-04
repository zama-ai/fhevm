import { ChainsRepository } from '#chains/domain/repositories/chains.repository.js'
import { beforeEach, describe, expect, test } from 'vitest'
import { ConfigChainsRepository } from './config-chains.repository.js'
import { TestBed } from '@suites/unit'
import { faker } from '@faker-js/faker'
import { Mocked } from '@suites/doubles.vitest'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import { Chain } from '#chains/domain/entities/chain.js'
import { ConfigService } from '@nestjs/config'

describe('ConfigChainsRepository', () => {
  let repo: ChainsRepository
  let config: Mocked<ConfigService>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      ConfigChainsRepository,
    ).compile()

    repo = unit
    config = unitRef.get(ConfigService) as unknown as Mocked<ConfigService>
  })

  test('should be defined', () => {
    expect(repo).toBeDefined()
  })

  describe('getChainById', () => {
    let chainId: number
    beforeEach(() => {
      chainId = faker.number.int({ min: 1, max: 100_000 })
    })

    describe('given no chain exists', () => {
      beforeEach(() => {
        config.get.mockReturnValue(undefined)
      })

      test('when is called, then it should return a not found error', async () => {
        await expect(
          repo.getChainById(ChainId.from(chainId).unwrap()).toPromise(),
        ).rejects.toThrowError(/not found/i)
      })
    })

    describe('given a chain exists', () => {
      beforeEach(() => {
        console.log(`calling mockImplementation on config.get`)
        config.get.mockReturnValue([
          {
            id: chainId,
            name: faker.string.alphanumeric(10),
            description: faker.lorem.words(5),
          },
        ])
      })

      test('when is called, then it should return the chain', async () => {
        const result = await repo
          .getChainById(ChainId.from(chainId).unwrap())
          .toPromise()
        expect(result.id.value).toEqual(chainId)
      })
    })
  })

  describe('getChains', () => {
    describe('given no chain exists', () => {
      beforeEach(() => {
        config.get.mockReturnValue(undefined)
      })

      test('when is called, then it should return an empty array', async () => {
        const result = await repo.getChains().toPromise()
        expect(result).toEqual([])
      })
    })

    describe('given chains exist', () => {
      let chains: Array<{ id: number; name: string; description?: string }>

      beforeEach(() => {
        chains = [
          {
            id: faker.number.int({ min: 1, max: 100_000 }),
            name: faker.string.alphanumeric(10),
            description: faker.lorem.words(5),
          },
          {
            id: faker.number.int({ min: 1, max: 100_000 }),
            name: faker.string.alphanumeric(10),
          },
          {
            id: faker.number.int({ min: 1, max: 100_000 }),
            name: faker.string.alphanumeric(10),
          },
        ]

        config.get.mockReturnValue(chains)
      })

      test('when is called, then it should return the chains', async () => {
        const result = await repo.getChains().toPromise()
        expect(result.length).toEqual(chains.length)
        result.forEach((chain, index) => {
          expect(chain).toEqual(Chain.parse(chains[index]).unwrap())
        })
      })
    })
  })
})
