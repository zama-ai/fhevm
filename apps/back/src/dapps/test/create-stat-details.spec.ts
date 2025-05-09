import { faker } from '@faker-js/faker'
import { describe, expect, test } from 'vitest'
import { StoreDAppStats } from '../use-cases/store-dapp-stats.use-case.js'
import { DAppId } from '#dapps/domain/entities/value-objects.js'
import { StatsType } from '#prisma/client/index.js'
import { operationNames } from 'messages'

describe('StoreDAppStats.createStatDetails', () => {
  test('should create stat details with correct date calculations', () => {
    const dappId = DAppId.random().value
    const timestamp = '2024-03-15T10:30:00Z'
    const event = {
      name: faker.helpers.arrayElement(operationNames),
      timestamp,
      externalRef: faker.string.uuid(),
    }

    const result = StoreDAppStats.createStatDetails(event, dappId)

    const date = new Date(timestamp)
    const expectedDay =
      (Date.UTC(2024, 2, 15) - Date.UTC(2024, 0, 0)) / (1000 * 60 * 60 * 24)

    expect(result).toEqual({
      id: expect.any(String),
      dappId,
      type: StatsType.COMPUTATION,
      day: expectedDay,
      month: date.getUTCMonth(),
      year: date.getUTCFullYear(),
      name: event.name,
      timestamp: date,
      externalRef: event.externalRef,
    })
  })

  test('should handle different dates correctly', () => {
    const dappId = DAppId.random().value
    const timestamp = '2024-01-01T00:00:00Z'
    const event = {
      name: faker.helpers.arrayElement(operationNames),
      timestamp,
      externalRef: faker.string.uuid(),
    }

    const result = StoreDAppStats.createStatDetails(event, dappId)

    const date = new Date(timestamp)
    const expectedDay =
      (Date.UTC(2024, 0, 1) - Date.UTC(2024, 0, 0)) / (1000 * 60 * 60 * 24)

    expect(result).toEqual({
      id: expect.any(String),
      dappId,
      type: StatsType.COMPUTATION,
      day: expectedDay,
      month: date.getUTCMonth(),
      year: date.getUTCFullYear(),
      name: event.name,
      timestamp: date,
      externalRef: event.externalRef,
    })
  })

  test('should handle leap year dates correctly', () => {
    const dappId = DAppId.random().value
    const timestamp = '2024-02-29T00:00:00Z'
    const event = {
      name: faker.helpers.arrayElement(operationNames),
      timestamp,
      externalRef: faker.string.uuid(),
    }

    const result = StoreDAppStats.createStatDetails(event, dappId)

    const date = new Date(timestamp)
    const expectedDay =
      (Date.UTC(2024, 1, 29) - Date.UTC(2024, 0, 0)) / (1000 * 60 * 60 * 24)

    expect(result).toEqual({
      id: expect.any(String),
      dappId,
      type: StatsType.COMPUTATION,
      day: expectedDay,
      month: date.getUTCMonth(),
      year: date.getUTCFullYear(),
      name: event.name,
      timestamp: date,
      externalRef: event.externalRef,
    })
  })
})
