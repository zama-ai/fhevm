import { TestBed } from '@suites/unit'
import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { DeleteResetPasswordToken } from './delete-reset-password-token.use-case.js'
import {
  PASSWORD_RESET_TOKEN_REPOSITORY,
  PasswordResetTokenRepository,
} from '#auth/domain/repositories/password-reset-token.repository.js'
import { notFoundError, Task } from 'utils'
import { Token } from '#auth/domain/entities/value-objects/token.js'
import { Hash } from '#auth/domain/entities/value-objects/hash.js'
import { faker } from '@faker-js/faker'

describe('DeleteResetPasswordToken', () => {
  let useCase: DeleteResetPasswordToken
  let repo: Mocked<PasswordResetTokenRepository>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      DeleteResetPasswordToken,
    ).compile()

    useCase = unit
    repo = unitRef.get(
      PASSWORD_RESET_TOKEN_REPOSITORY,
    ) as unknown as Mocked<PasswordResetTokenRepository>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given a password reset token exists', () => {
    let token: Token
    beforeEach(() => {
      token = Token.random()
      repo.deleteByHash.mockReturnValue(Task.of(void 0))
    })

    describe('when deleting the token', () => {
      let result: ReturnType<DeleteResetPasswordToken['execute']>

      beforeEach(() => {
        result = useCase.execute({ hash: Hash.hash(token).value })
      })

      test('then it should delete the token', async () => {
        await result.toPromise()
        expect(repo.deleteByHash).toHaveBeenCalledWith(Hash.hash(token))
      })
    })
  })

  describe('given a password reset token does not exists', () => {
    beforeEach(() => {
      repo.deleteByHash.mockReturnValue(Task.reject(notFoundError()))
    })

    describe('when deleting the token', () => {
      let result: ReturnType<DeleteResetPasswordToken['execute']>

      beforeEach(() => {
        result = useCase.execute({ hash: faker.string.alphanumeric() })
      })

      test('then it throw a not found error', async () => {
        await expect(result.toPromise()).rejects.toThrow(/not found/i)
      })
    })
  })
})
