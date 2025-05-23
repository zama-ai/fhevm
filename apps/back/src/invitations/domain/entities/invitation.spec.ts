import { faker } from '@faker-js/faker'
import { beforeEach, describe, expect, test } from 'vitest'
import { Invitation, InvitationProps } from './invitation.js'
import { BRAND } from 'zod'
import { ExpiresAt } from '#shared/entities/value-objects/expires-at.js'

describe('Invitation', () => {
  test('expiresAt', () => {
    try {
      const expiresAt = new ExpiresAt(new Date())
      expect(expiresAt).toBeDefined()
    } catch (err) {
      console.error(err)
      expect.fail()
    }
  })
  describe('parse', () => {
    let props: InvitationProps

    beforeEach(() => {
      props = {
        id: faker.string.uuid() as string & BRAND<'InvitationId'>,
        email: faker.internet.email(),
        token: faker.string.uuid() as string & BRAND<'Token'>,
        expiresAt: new Date() as Date & BRAND<'ExpiresAt'>,
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
        if (invitation.isFail()) {
          expect(invitation.error.message).toContain('id')
        }
      })

      test('email is not an email', () => {
        const invitation = Invitation.parse({
          ...props,
          email: faker.string.alphanumeric(),
        })
        expect(invitation.isFail()).toBe(true)
        if (invitation.isFail()) {
          expect(invitation.error.message).toContain('email')
        }
      })

      test('token is not an UUID', () => {
        const invitation = Invitation.parse({
          ...props,
          token: faker.string.alphanumeric(),
        })
        expect(invitation.isFail()).toBe(true)
        if (invitation.isFail()) {
          expect(invitation.error.message).toContain('token')
        }
      })

      test('expiresAt is null', () => {
        const invitation = Invitation.parse({
          ...props,
          expiresAt: null,
        })
        expect(invitation.isFail()).toBe(true)
        if (invitation.isFail()) {
          expect(invitation.error.message).toContain('expiresAt')
        }
      })
    })
  })

  describe('create', () => {
    test('should create an invitation', () => {
      const invitation = Invitation.create({
        email: faker.internet.email(),
      })
      expect(invitation.isOk()).toBe(true)
    })

    test('should generate an invitation id', () => {
      const invitation = Invitation.create({
        email: faker.internet.email(),
      })
      expect(invitation.isOk()).toBe(true)
      if (invitation.isOk()) {
        expect(invitation.value.id).toBeDefined()
      }
    })

    test('should generate an invitation token', () => {
      const invitation = Invitation.create({
        email: faker.internet.email(),
      })
      expect(invitation.isOk()).toBe(true)
      if (invitation.isOk()) {
        expect(invitation.value.token).toBeDefined()
      }
    })

    test('should generate an invitation expiresAt in the future', () => {
      const invitation = Invitation.create({
        email: faker.internet.email(),
      })
      expect(invitation.isOk()).toBe(true)
      if (invitation.isOk()) {
        expect(invitation.value.expiresAt).toBeDefined()
        expect(invitation.value.expiresAt.value.getTime()).toBeGreaterThan(
          Date.now(),
        )
      }
    })

    describe('should fail when', () => {
      test('email is not an email', () => {
        const invitation = Invitation.create({
          email: faker.string.alphanumeric(),
        })
        expect(invitation.isFail()).toBe(true)
        if (invitation.isFail()) {
          expect(invitation.error.message).toContain('email')
        }
      })
    })
  })
})
