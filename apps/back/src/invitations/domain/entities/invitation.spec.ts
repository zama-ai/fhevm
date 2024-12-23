import { faker } from '@faker-js/faker'
import { beforeEach, describe, expect, test } from 'vitest'
import { Invitation, InvitationProps } from './invitation'
import { BRAND } from 'zod'

describe('Invitation', () => {
  describe('parse', () => {
    let props: InvitationProps

    beforeEach(() => {
      props = {
        id: faker.string.uuid() as string & BRAND<'InvitationId'>,
        email: faker.internet.email(),
        token: faker.string.uuid() as string & BRAND<'Token'>,
        expiresAt: new Date(),
      }
    })
    test('should parse a valid invitation', () => {
      const invitation = Invitation.parse(props)
      expect(invitation.isOk()).toBe(true)
    })

    describe('should fail when', () => {
      test('data is undefined', () => {
        const invitation = Invitation.parse(undefined)
        expect(invitation.isFail()).toBe(true)
      })

      test('id is not an UUID', () => {
        const invitation = Invitation.parse({
          ...props,
          id: faker.string.alphanumeric(),
        })
        expect(invitation.isFail()).toBe(true)
      })

      test('email is not an email', () => {
        const invitation = Invitation.parse({
          ...props,
          email: faker.string.alphanumeric(),
        })
        expect(invitation.isFail()).toBe(true)
      })

      test('token is not an UUID', () => {
        const invitation = Invitation.parse({
          ...props,
          token: faker.string.alphanumeric(),
        })
        expect(invitation.isFail()).toBe(true)
      })

      test('expiresAt is null', () => {
        const invitation = Invitation.parse({
          ...props,
          expiresAt: null,
        })
        expect(invitation.isFail()).toBe(true)
      })
    })
  })
})
