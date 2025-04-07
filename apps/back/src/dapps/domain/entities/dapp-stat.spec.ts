import { beforeEach, describe, expect, test } from 'vitest'
import { DAppId, DAppStatId } from './value-objects.js'
import { faker } from '@faker-js/faker'
import { DAppStat } from './dapp-stat.js'
import { z } from 'zod'
import { StatsType } from '#prisma/client/index.js'

describe('DappStat', () => {
  describe('parse', () => {
    let params: Record<string, unknown>

    beforeEach(() => {
      params = {
        id: DAppStatId.random().value,
        name: 'FheAdd',
        type: StatsType.COMPUTATION,
        day: faker.number.int({ min: 1, max: 365 }),
        month: faker.number.int({ min: 0, max: 11 }),
        year: faker.number.int({ min: 2020, max: 2025 }),
        timestamp: new Date(Date.now()),
        dappId: DAppId.random().value,
        externalRef: faker.string.alphanumeric(10),
      }
    })

    test('should parse a valid dappStat', () => {
      const result = DAppStat.parse(params)
      if (result.isFail()) {
        console.error(`Failed to parse dappStat: ${result.error.message}`)
      }
      expect(result.isOk()).toBe(true)
    })

    describe('should fail when', () => {
      describe('id', () => {
        test('is missing', () => {
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
          const { id, ...data } = params
          const result = DAppStat.parse(data)
          expect(result.isFail()).toBe(true)
        })

        test('does not start with stat_', () => {
          const result = DAppStat.parse({
            ...params,
            id: faker.string.alphanumeric(),
          })
          expect(result.isFail()).toBe(true)
        })

        test('is not 22 characters long', () => {
          for (const l of [21, 23]) {
            const result = DAppStat.parse({
              ...params,
              id: `stat_${faker.string.alphanumeric(l - 5)}`,
            })
            expect(result.isFail()).toBe(true)
          }
        })
      })

      describe('name', () => {
        // Note: rework name to an enum
        test('is missing', () => {
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
          const { name, ...data } = params
          const result = DAppStat.parse(data)
          expect(result.isFail()).toBe(true)
        })
      })

      describe('timestamp', () => {
        test('is missing', () => {
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
          const { timestamp, ...data } = params
          const result = DAppStat.parse(data)
          expect(result.isFail()).toBe(true)
        })
      })

      describe('dappId', () => {
        test('is missing', () => {
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
          const { dappId, ...data } = params
          const result = DAppStat.parse(data)
          expect(result.isFail()).toBe(true)
        })

        test('is invalid', () => {
          const result = DAppStat.parse({
            ...params,
            dappId: faker.string.alphanumeric(16),
          })
          expect(result.isFail()).toBe(true)
        })
      })

      describe('externalRef', () => {
        test('is missing', () => {
          // eslint-disable-next-line @typescript-eslint/no-unused-vars
          const { externalRef, ...data } = params
          const result = DAppStat.parse(data)
          expect(result.isFail()).toBe(true)
        })
      })
    })
  })

  describe('create', () => {
    let stat: DAppStat

    beforeEach(() => {
      const result = DAppStat.create({
        name: 'FheAdd',
        timestamp: new Date(Date.now()),
        type: StatsType.COMPUTATION,
        day: faker.number.int({ min: 1, max: 365 }),
        month: faker.number.int({ min: 0, max: 11 }),
        year: faker.number.int({ min: 2020, max: 2025 }),
        // Note: to fix
        dappId: DAppId.random().value as `dapp_${string}` & z.BRAND<'DAppId'>,
        externalRef: faker.string.alphanumeric(10),
      })
      expect(result.isOk()).toBe(true)
      stat = result.unwrap()
    })

    test('should create a dappStat', () => {
      expect(stat).toBeDefined()
    })

    test('should generate a dappStat id', () => {
      expect(stat.id.value).toBeTruthy()
    })
  })
})
