import { Test } from '@nestjs/testing'
import { beforeEach, describe, expect, test, vi } from 'vitest'
import { PrismaDAppRepository } from './prisma-dapp.repository.js'
import { PrismaService } from '../__mocks__/prisma.service.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import { DAppId, DAppStatId } from '#dapps/domain/entities/value-objects.js'
import { DAppStat, DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { AppError, every, executeTask, isAppError } from 'utils'
import { faker } from '@faker-js/faker'
import { z } from 'zod'

vi.mock('../prisma.service.js')

describe('PrismaDappRepository', () => {
  let repo: DAppRepository
  let prisma: PrismaService

  beforeEach(async () => {
    const moduleRef = await Test.createTestingModule({
      providers: [PrismaDAppRepository, PrismaService],
    }).compile()

    repo = moduleRef.get(PrismaDAppRepository)
    prisma = moduleRef.get(PrismaService)
  })

  describe('findAllStats', () => {
    describe('given no stats exist', () => {
      describe('when is called', () => {
        beforeEach(() => {
          prisma.dappStat.findMany.mockResolvedValue([])
        })

        test('should return an empty array', async () => {
          const {
            success,
            value: stats,
            error,
          } = await executeTask<DAppStat[], AppError>(
            repo.findAllStats(DAppId.random()),
          )
          expect(success).toBe(true)
          expect(stats).toEqual([])
          expect(error).toBeUndefined()
        })
      })

      describe('when raises an error', () => {
        beforeEach(async () => {
          prisma.dappStat.findMany.mockRejectedValue(new Error('mocked error'))
        })

        test('should return an error', async () => {
          const {
            success,
            value: stats,
            error,
          } = await executeTask<DAppStat[], AppError>(
            repo.findAllStats(DAppId.random()),
          )
          expect(success).toBe(false)
          expect(stats).toBeUndefined()
          expect(isAppError(error)).toBe(true)
        })
      })
    })

    describe('given stats exist', () => {
      const stats: DAppStatProps[] = [
        {
          id: DAppStatId.random().value as string & z.BRAND<'DAppStatId'>,
          name: faker.string.alphanumeric(10),
          timestamp: new Date(Date.now()),
          dappId: DAppId.random().value as `dapp_${string}` & z.BRAND<'DAppId'>,
          externalRef: faker.string.alphanumeric(10),
        },
      ]
      describe('when is called', () => {
        beforeEach(() => {
          prisma.dappStat.findMany.mockResolvedValue(stats)
        })

        test('should return stats', async () => {
          const { success, value, error } = await executeTask<
            DAppStat[],
            AppError
          >(repo.findAllStats(DAppId.random()))
          expect(success).toBe(true)
          expect(value).toEqual(every(stats.map(DAppStat.create)).unwrap())
          expect(error).toBeUndefined()
        })
      })

      describe('when raises an error', () => {
        beforeEach(async () => {
          prisma.dappStat.findMany.mockRejectedValue(new Error('mocked error'))
        })

        test('should return an error', async () => {
          const {
            success,
            value: stats,
            error,
          } = await executeTask<DAppStat[], AppError>(
            repo.findAllStats(DAppId.random()),
          )
          expect(success).toBe(false)
          expect(stats).toBeUndefined()
          expect(isAppError(error)).toBe(true)
        })
      })

      describe('when a stat is invalid', () => {
        beforeEach(() => {
          prisma.dappStat.findMany.mockResolvedValue([
            {
              id: DAppStatId.random().value as string & z.BRAND<'DAppStatId'>,
              name: faker.string.alphanumeric(10),
              timestamp: new Date(Date.now()),
              dappId: DAppId.random().value as `dapp_${string}` &
                z.BRAND<'DAppId'>,
              externalRef: faker.string.alphanumeric(16),
            },
            {
              id: 'invalid_id',
              name: faker.string.alphanumeric(10),
              timestamp: new Date(Date.now()),
              dappId: DAppId.random().value as `dapp_${string}` &
                z.BRAND<'DAppId'>,
              externalRef: faker.string.alphanumeric(16),
            },
          ])
        })

        test('should return fail', async () => {
          const { success, value, error } = await executeTask<
            DAppStat[],
            AppError
          >(repo.findAllStats(DAppId.random()))
          expect(success).toBe(false)
          expect(value).toBeUndefined()
          expect(isAppError(error)).toBe(true)
        })
      })
    })
  })
})
