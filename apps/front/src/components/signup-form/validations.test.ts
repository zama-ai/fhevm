import { getPasswordStrengthScore } from './validations'

describe('getPasswordStrengthScore', () => {
  it('should return 0 for passwords shorter than 8 characters', () => {
    expect(getPasswordStrengthScore('short')).toBe(0)
  })

  it('should return 1 for passwords with at least 8 characters', () => {
    expect(getPasswordStrengthScore('longenough')).toBe(1)
  })

  it('should return 2 for passwords with both lowercase and uppercase characters', () => {
    expect(getPasswordStrengthScore('Longenough')).toBe(2)
  })

  it('should return 3 for passwords with lowercase, uppercase characters, and digits', () => {
    expect(getPasswordStrengthScore('Longenough1')).toBe(3)
  })

  it('should return 4 for passwords with lowercase, uppercase characters, digits, and special characters', () => {
    expect(getPasswordStrengthScore('Longenough1!')).toBe(4)
  })
})
