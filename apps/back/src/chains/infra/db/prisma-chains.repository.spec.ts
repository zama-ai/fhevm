import { ChainsRepository } from '#chains/domain/repositories/chains.repository.js'
import { beforeEach, describe, expect, test } from 'vitest'
import { PrismaChainsRepository } from './prisma-chains.repository.js'
import { TestBed } from '@suites/unit'
import { faker } from '@faker-js/faker'
import { Mocked } from '@suites/doubles.vitest'
import { PrismaService } from '#infra/database/prisma.service.js'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import { Chain as PrismaChain } from '#prisma/client/index.js'
import { Chain } from '#chains/domain/entities/chain.js'

describe('PrismaChainsRepository', () => {
  let repo: ChainsRepository
  let prisma: Mocked<PrismaService>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      PrismaChainsRepository,
    ).compile()

    repo = unit
    prisma = unitRef.get(PrismaService) as unknown as Mocked<PrismaService>
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
        prisma.chain.findUnique.mockResolvedValue(null)
      })

      test('when is called, then it should return a not found error', async () => {
        await expect(
          repo.getChainById(ChainId.from(chainId).unwrap()).toPromise(),
        ).rejects.toThrowError(/not found/i)
        expect(prisma.chain.findUnique).toHaveBeenCalledWith({
          where: {
            id: chainId,
            enabled: true,
          },
        })
      })
    })

    describe('given a chain exists', () => {
      let chain: PrismaChain

      beforeEach(() => {
        chain = {
          id: chainId,
          name: faker.string.alphanumeric(10),
          description: faker.lorem.words(5),
          enabled: true,
        }

        prisma.chain.findUnique.mockResolvedValue(chain)
      })

      test('when is called, then it should return the chain', async () => {
        const result = await repo
          .getChainById(ChainId.from(chainId).unwrap())
          .toPromise()
        expect(result).toEqual(Chain.parse(chain).unwrap())
        expect(prisma.chain.findUnique).toHaveBeenCalledWith({
          where: {
            id: chainId,
            enabled: true,
          },
        })
      })
    })
  })

  describe('getChains', () => {
    describe('given no chain exists', () => {
      beforeEach(() => {
        prisma.chain.findMany.mockResolvedValue([])
      })

      test('when is called, then it should return an empty array', async () => {
        const result = await repo.getChains().toPromise()
        expect(result).toEqual([])
        expect(prisma.chain.findMany).toHaveBeenCalledWith({
          where: {
            enabled: true,
          },
        })
      })
    })

    describe('given chains exist', () => {
      let chains: PrismaChain[]

      beforeEach(() => {
        chains = [
          {
            id: faker.number.int({ min: 1, max: 100_000 }),
            name: faker.string.alphanumeric(10),
            description: faker.lorem.words(5),
            enabled: true,
          },
          {
            id: faker.number.int({ min: 1, max: 100_000 }),
            name: faker.string.alphanumeric(10),
            description: null,
            enabled: true,
          },
          {
            id: faker.number.int({ min: 1, max: 100_000 }),
            name: faker.string.alphanumeric(10),
            description: null,
            enabled: true,
          },
        ]

        prisma.chain.findMany.mockResolvedValue(chains)
      })

      test('when is called, then it should return the chains', async () => {
        const result = await repo.getChains().toPromise()
        expect(result.length).toEqual(chains.length)
        result.forEach((chain, index) => {
          expect(chain).toEqual(Chain.parse(chains[index]).unwrap())
        })
        expect(prisma.chain.findMany).toHaveBeenCalledWith({
          where: {
            enabled: true,
          },
        })
      })
    })
  })
})
