import { beforeEach, describe, expect, test } from 'vitest'
import { faker } from '@faker-js/faker'
import { newOrganisationName } from './user-registered.logic.js'
import type { UserRegistered } from './webhooks.types.js'

describe('newOrganizationName', () => {
  describe('should return the expected name', () => {
    let event: UserRegistered
    beforeEach(() => {
      event = {
        id: faker.number.int({ min: 1 }),
        email: faker.internet.email(),
        first: faker.person.firstName(),
        last: faker.person.lastName(),
        orgId: faker.number.int({ min: 1 }),
        provider: faker.string.alphanumeric(10),
        createdAt: faker.date.past().toISOString(),
        customAttributes: [
          {
            identifier: faker.string.alphanumeric(10),
            value: faker.string.alphanumeric(10),
          },
        ],
      } satisfies UserRegistered
    })

    test('when the payload is valid', () => {
      expect(newOrganisationName(event)).toEqual(
        `${event.first} ${event.last}'s organisation`,
      )
    })

    test('when the first name is missing', () => {
      event.first = ''
      expect(newOrganisationName(event)).toEqual(`${event.last}'s organisation`)
    })

    test('when the last name is missing', () => {
      event.last = ''
      expect(newOrganisationName(event)).toEqual(
        `${event.first}'s organisation`,
      )
    })
  })
})
