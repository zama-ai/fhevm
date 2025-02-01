import { faker } from '@faker-js/faker'
import { beforeEach, describe, expect, test } from 'vitest'
import { DApp } from './dapp.js'
import { TeamId } from '#users/domain/entities/value-objects.js'
import { DAppId } from './value-objects.js'

describe('Dapp', () => {
  describe('parse', () => {
    let params: Record<string, unknown>

    beforeEach(() => {
      params = {
        id: DAppId.random().value,
        name: faker.string.alphanumeric(10),
        status: faker.helpers.arrayElement(['DRAFT', 'DEPLOYING', 'LIVE']),
        teamId: TeamId.random().value,
        createdAt: new Date(Date.now() - 1000),
      }
    })
    test('should parse a valid dapp', () => {
      const result = DApp.parse(params)
      expect(result.isOk()).toBe(true)
    })

    describe('should fail when', () => {
      test('id is not a uuid', () => {
        const result = DApp.parse({
          ...params,
          id: faker.string.alphanumeric(),
        })
        expect(result.isFail()).toBe(true)
        if (result.isFail()) {
          expect(result.error.message).toContain('id')
        }
      })

      test('name is not a string', () => {
        const result = DApp.parse({ ...params, name: 123 })
        expect(result.isFail()).toBe(true)
        if (result.isFail()) {
          expect(result.error.message).toContain('name')
        }
      })

      test('status is not a valid status', () => {
        const result = DApp.parse({
          ...params,
          status: faker.string.alphanumeric(),
        })
        expect(result.isFail()).toBe(true)
        if (result.isFail()) {
          expect(result.error.message).toContain('status')
        }
      })

      test('teamId is not a uuid', () => {
        const result = DApp.parse({
          ...params,
          teamId: faker.string.alphanumeric(),
        })
        expect(result.isFail()).toBe(true)
        if (result.isFail()) {
          expect(result.error.message).toContain('teamId')
        }
      })

      test('address is not a valid Contract address', () => {
        const result = DApp.parse({
          ...params,
          address: faker.string.alphanumeric(),
        })
        expect(result.isFail()).toBe(true)
        if (result.isFail()) {
          expect(result.error.message).toContain('address')
        }
      })

      test('createdAt is not a date', () => {
        const result = DApp.parse({
          ...params,
          createdAt: faker.string.alphanumeric(),
        })
        expect(result.isFail()).toBe(true)
        if (result.isFail()) {
          expect(result.error.message).toContain('createdAt')
        }
      })

      test('createdAt is in the future', () => {
        const result = DApp.parse({
          ...params,
          createdAt: new Date(Date.now() + 1000),
        })
        expect(result.isFail()).toBe(true)
        if (result.isFail()) {
          expect(result.error.message).toContain('createdAt')
        }
      })
    })
  })

  describe('create', () => {
    let dapp: DApp

    beforeEach(() => {
      const result = DApp.create({
        name: faker.string.alphanumeric(10),
        teamId: TeamId.random().value,
      })
      expect(result.isOk()).toBe(true)
      dapp = result.unwrap()
    })
    test('should create a dapp', () => {
      expect(dapp).toBeDefined()
    })

    test('should generate a dapp id', () => {
      expect(dapp.id.value).toBeTruthy()
    })

    test('status should be DRAFT', () => {
      expect(dapp.status).toBe('DRAFT')
    })

    test('createdAt should be now', () => {
      expect(dapp.createdAt.getTime()).toBeLessThanOrEqual(Date.now())
      expect(dapp.createdAt.getTime()).toBeGreaterThanOrEqual(Date.now() - 100)
    })
  })
})
