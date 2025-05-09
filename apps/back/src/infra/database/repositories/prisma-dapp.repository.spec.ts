import { beforeEach, describe, expect, test, vi } from 'vitest'
import { PrismaDAppRepository } from './prisma-dapp.repository.js'
import { DAppRepository } from '#dapps/domain/repositories/dapp.repository.js'
import {
  ApiKeyId,
  DAppId,
  DAppStatId,
  Token,
} from '#dapps/domain/entities/value-objects.js'
import { DAppStat, DAppStatProps } from '#dapps/domain/entities/dapp-stat.js'
import { AppError, every, executeTask, isAppError } from 'utils'
import { faker } from '@faker-js/faker'
import { z } from 'zod'
import { TeamId } from '#users/domain/entities/value-objects.js'
import { Dapp, DappStatus, StatsType } from '#prisma/client/index.js'
import { ApiKey } from '#dapps/domain/entities/api-key.js'
import { TestBed } from '@suites/unit'
import { Mocked } from '@suites/doubles.vitest'
import { PrismaService } from '../prisma.service.js'
import { ChainId } from '#chains/domain/entities/value-objects.js'

vi.mock('../prisma.service.js')

describe('PrismaDappRepository', () => {
  let repo: DAppRepository
  let prisma: Mocked<PrismaService>

  beforeEach(async () => {
    const { unit, unitRef } =
      await TestBed.solitary(PrismaDAppRepository).compile()

    repo = unit
    prisma = unitRef.get(PrismaService) as unknown as Mocked<PrismaService>
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
          name: 'FheAdd',
          timestamp: new Date(Date.now()),
          dappId: DAppId.random().value as `dapp_${string}` & z.BRAND<'DAppId'>,
          type: StatsType.COMPUTATION,
          day: 1,
          month: 0,
          year: 2024,
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
              type: StatsType.COMPUTATION,
              day: 1,
              month: 0,
              year: 2024,
              externalRef: faker.string.alphanumeric(16),
            },
            {
              id: 'invalid_id',
              name: faker.string.alphanumeric(10),
              timestamp: new Date(Date.now()),
              dappId: DAppId.random().value as `dapp_${string}` &
                z.BRAND<'DAppId'>,
              type: StatsType.COMPUTATION,
              day: 1,
              month: 0,
              year: 2024,
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

  describe('delete', () => {
    describe('given a dapp exists', () => {
      let dappId: DAppId
      let dapp: Dapp
      beforeEach(() => {
        dappId = DAppId.random()
        dapp = {
          id: dappId.value,
          name: faker.string.alphanumeric(10),
          status: 'LIVE',
          chainId: ChainId.from(
            faker.number.int({ min: 1, max: 100_000 }),
          ).unwrap().value,
          address: faker.string.hexadecimal({ length: 40 }),
          teamId: TeamId.random().value,
          updatedAt: faker.date.past(),
          createdAt: faker.date.past(),
          deletedAt: null,
        }
        prisma.dapp.findUnique.mockResolvedValue(dapp)
        prisma.dapp.findFirst.mockResolvedValue(dapp)
      })

      describe('when I delete it', () => {
        beforeEach(() => {
          prisma.dapp.update.mockResolvedValue({
            ...dapp,
            deletedAt: new Date(),
          })
        })

        test('should soft delete it', async () => {
          const startedAt = new Date()
          const { success } = await executeTask<void, AppError>(
            repo.delete(dappId),
          )
          expect(success).toBe(true)
          expect(prisma.dapp.update).toHaveBeenCalledExactlyOnceWith({
            data: { deletedAt: expect.any(Date) },
            where: { id: dappId.value },
          })
          expect(
            prisma.dapp.update.mock.calls[0][0].data.deletedAt,
          ).greaterThanOrEqual(startedAt)
          expect(
            prisma.dapp.update.mock.calls[0][0].data.deletedAt,
          ).lessThanOrEqual(new Date())
        })
      })
    })

    describe(`given a dapp doesn't exist`, () => {
      let dappId: DAppId
      beforeEach(() => {
        dappId = DAppId.random()

        prisma.dapp.findUnique.mockResolvedValue(null)
        prisma.dapp.findFirst.mockResolvedValue(null)
      })

      describe('when I delete it', () => {
        test('should fail', async () => {
          const { success, error } = await executeTask<void, AppError>(
            repo.delete(dappId),
          )
          expect(success).toBe(false)
          expect(error?._tag).toBe('NotFoundError')
          expect(prisma.dapp.update).not.toHaveBeenCalled()
        })
      })
    })
  })

  describe('createApiKey', () => {
    describe('given a DApp exists', () => {
      let dappId: DAppId
      beforeEach(() => {
        dappId = DAppId.random()

        const props = {
          id: dappId.value,
          name: faker.string.alphanumeric(10),
          status: 'LIVE' as DappStatus,
          chainId: ChainId.from(
            faker.number.int({ min: 1, max: 100_000 }),
          ).unwrap().value,
          address: faker.string.hexadecimal({ length: 40 }),
          teamId: TeamId.random().value,
          updatedAt: faker.date.past(),
          createdAt: faker.date.past(),
          deletedAt: null,
        }
        prisma.dapp.findUnique.mockResolvedValue(props)
        prisma.dapp.findFirst.mockResolvedValue(props)
      })

      describe('when I create a api key', () => {
        let apiKeyId: ApiKeyId
        let token: Token
        beforeEach(() => {
          apiKeyId = ApiKeyId.random()
          token = Token.random()
          prisma.apiKey.create.mockResolvedValue({
            id: apiKeyId.value,
            token: token.value,
            dappId: dappId.value,
            name: faker.string.alphanumeric(10),
            description: faker.lorem.sentence(),
            createdAt: new Date(),
            deletedAt: null,
          })
        })

        test('then it should create a new api key', async () => {
          const apiKey = ApiKey.parse({
            id: apiKeyId.value,
            token: token.value,
            dappId: dappId.value,
            name: faker.string.alphanumeric(10),
            createdAt: faker.date.past(),
          }).unwrap()
          const { success, value } = await executeTask(
            repo.createApiKey(apiKey),
          )
          expect(success).toBe(true)
          expect(value?.id.value).toBe(apiKeyId.value)
          expect(value?.dappId.value).toBe(dappId.value)
        })
      })
    })

    describe(`given a DApp doesn't exist`, () => {
      describe('when I create a api key', () => {
        let apiKeyId: ApiKeyId
        let token: Token
        let dappId: DAppId
        beforeEach(() => {
          apiKeyId = ApiKeyId.random()
          dappId = DAppId.random()
          token = Token.random()

          prisma.dapp.findUnique.mockResolvedValue(null)
          prisma.apiKey.create.mockResolvedValue({
            id: apiKeyId.value,
            token: token.value,
            dappId: dappId.value,
            name: faker.string.alphanumeric(10),
            description: null,
            createdAt: new Date(),
            deletedAt: null,
          })
        })

        test('then it should fail', async () => {
          const apiKey = ApiKey.parse({
            id: apiKeyId.value,
            token: token.value,
            dappId: dappId.value,
            name: faker.string.alphanumeric(10),
            createdAt: faker.date.past(),
          }).unwrap()
          const { success, error } = await executeTask(
            repo.createApiKey(apiKey),
          )
          expect(success).toBe(false)
          expect(error?.message).toContain('DApp not found')
        })
      })
    })
  })

  describe('findAllApiKeys', () => {
    describe('given a DApp exists', () => {
      let dappId: DAppId
      beforeEach(() => {
        dappId = DAppId.random()
        prisma.dapp.findUnique.mockResolvedValue({
          id: dappId.value,
          name: faker.string.alphanumeric(10),
          status: 'LIVE' as DappStatus,
          chainId: ChainId.from(
            faker.number.int({ min: 1, max: 100_000 }),
          ).unwrap().value,
          address: faker.string.hexadecimal({ length: 40 }),
          teamId: TeamId.random().value,
          updatedAt: faker.date.past(),
          createdAt: faker.date.past(),
          deletedAt: null,
        })
      })

      describe('when I request all api keys', () => {
        let apiKeys: ApiKey[]
        beforeEach(() => {
          apiKeys = [
            ApiKey.parse({
              id: ApiKeyId.random().value,
              token: Token.random().value,
              dappId: dappId.value,
              name: faker.string.alphanumeric(10),
              createdAt: faker.date.past(),
            }).unwrap(),
            ApiKey.parse({
              id: ApiKeyId.random().value,
              token: Token.random().value,
              dappId: dappId.value,
              name: faker.string.alphanumeric(10),
              createdAt: faker.date.past(),
            }).unwrap(),
          ]
          prisma.apiKey.findMany.mockResolvedValue(
            apiKeys.map(apiKey => ({
              id: apiKey.id.value,
              token: apiKey.token.value,
              dappId: apiKey.dappId.value,
              name: faker.string.alphanumeric(10),
              description: null,
              createdAt: new Date(),
              deletedAt: null,
            })),
          )
        })

        test('then it should return all api keys', async () => {
          const { success, value } = await executeTask(
            repo.findAllApiKeys(dappId),
          )
          expect(success).toBe(true)
          expect(value?.length).toBe(apiKeys.length)
          for (let i = 0; i < apiKeys.length; i++) {
            expect(value?.[i].id.value).toBe(apiKeys[i].id.value)
            expect(value?.[i].dappId.value).toBe(apiKeys[i].dappId.value)
          }
        })
      })
    })

    describe(`given a DApp doesn't exist`, () => {
      let dappId: DAppId
      beforeEach(() => {
        dappId = DAppId.random()
        prisma.dapp.findUnique.mockResolvedValue(null)
      })

      describe('when I request all api keys', () => {
        beforeEach(() => {
          prisma.apiKey.findMany.mockResolvedValue([])
        })

        test('then it should return an empty array', async () => {
          const { success, value } = await executeTask(
            repo.findAllApiKeys(dappId),
          )
          expect(success).toBe(true)
          expect(value?.length).toBe(0)
        })
      })
    })
  })
})
