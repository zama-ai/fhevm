import { describe, expect, test } from 'vitest'
import { ChangePasswordSchema, ChangePasswordValues } from './schema'
import { faker } from '@faker-js/faker'

describe('ChangePasswordSchema', () => {
  describe('newPassword', () => {
    test('should be at least 8 characters long', () => {
      const object = {
        oldPassword: faker.internet.password(),
        newPassword: faker.internet.password({ length: 7 }),
        repeatPassword: faker.internet.password(),
      } satisfies ChangePasswordValues

      const parsed = ChangePasswordSchema.safeParse(object)
      expect(parsed.success).toBe(false)
      if (!parsed.success) {
        const issue = parsed.error.issues.find(i => i.path[0] === 'newPassword')
        expect(issue).toBeDefined()
        expect(issue?.message).toContain('at least 8 characters long')
      }
    })

    test('should contain a least a lowercase character', () => {
      const object = {
        oldPassword: faker.internet.password(),
        newPassword: faker.internet.password({ length: 8 }).toUpperCase(),
        repeatPassword: faker.internet.password(),
      } satisfies ChangePasswordValues

      const parsed = ChangePasswordSchema.safeParse(object)
      expect(parsed.success).toBe(false)
      if (!parsed.success) {
        const issue = parsed.error.issues.find(i => i.path[0] === 'newPassword')
        expect(issue).toBeDefined()
        expect(issue?.message).toContain('at least a lowercase character')
      }
    })

    test('should contain a least an uppercase character', () => {
      const object = {
        oldPassword: faker.internet.password(),
        newPassword: faker.internet.password({ length: 8 }).toLowerCase(),
        repeatPassword: faker.internet.password(),
      } satisfies ChangePasswordValues

      const parsed = ChangePasswordSchema.safeParse(object)
      expect(parsed.success).toBe(false)
      if (!parsed.success) {
        const issue = parsed.error.issues.find(i => i.path[0] === 'newPassword')
        expect(issue).toBeDefined()
        expect(issue?.message).toContain('at least an uppercase character')
      }
    })

    test('should contain a least a special character', () => {
      const object = {
        oldPassword: faker.internet.password(),
        newPassword: faker.string.alphanumeric({ length: 8 }),
        repeatPassword: faker.internet.password(),
      } satisfies ChangePasswordValues

      const parsed = ChangePasswordSchema.safeParse(object)
      expect(parsed.success).toBe(false)
      if (!parsed.success) {
        const issue = parsed.error.issues.find(i => i.path[0] === 'newPassword')
        expect(issue).toBeDefined()
        expect(issue?.message).toContain('at least a special character')
      }
    })
  })

  describe('repeatPassword', () => {
    test('should match newPassword', () => {
      const object = {
        oldPassword: faker.internet.password(),
        newPassword: faker.internet.password(),
        repeatPassword: faker.internet.password(),
      } satisfies ChangePasswordValues

      const parsed = ChangePasswordSchema.safeParse(object)
      expect(parsed.success).toBe(false)
      if (!parsed.success) {
        const issue = parsed.error.issues.find(
          i => i.path[0] === 'repeatPassword',
        )
        expect(issue).toBeDefined()
        expect(issue?.message).toContain('Password does not match')
      }
    })
  })

  test('should be valid', () => {
    const password = faker.internet.password() + '!@#$'
    const object = {
      oldPassword: faker.internet.password(),
      newPassword: password,
      repeatPassword: password,
    } satisfies ChangePasswordValues

    const parsed = ChangePasswordSchema.safeParse(object)
    if (!parsed.success) {
      console.log(parsed.error)
    }
    expect(parsed.success).toBe(true)
  })
})
