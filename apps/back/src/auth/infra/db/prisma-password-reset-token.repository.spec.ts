import { PasswordResetTokenRepository } from '#auth/domain/repositories/password-reset-token.repository.js'
import { beforeEach, describe, expect, test } from 'vitest'
import { PrismaPasswordResetTokenRepository } from './prisma-password-reset-token.repository.js'
import { PrismaService } from '#infra/database/prisma.service.js'
import { Mocked } from '@suites/doubles.vitest'
import { TestBed } from '@suites/unit'
import { UserId } from '#users/domain/entities/value-objects.js'
import { PasswordResetToken } from '#auth/domain/entities/password-reset-token.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'

describe('PrismaPasswordResetTokenRepository', () => {
  let repo: PasswordResetTokenRepository
  let prisma: Mocked<PrismaService>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      PrismaPasswordResetTokenRepository,
    ).compile()

    repo = unit
    prisma = unitRef.get(PrismaService) as unknown as Mocked<PrismaService>
  })

  describe('create', () => {
    describe('given the user exists', () => {
      describe('when creating the token', () => {
        test('then it should create a password reset token', async () => {
          // Arrange
          const userId = UserId.random()
          const token = PasswordResetToken.create({
            token: Token.random(),
            userId,
          }).unwrap()

          prisma.passwordResetToken.upsert.mockResolvedValue({
            tokenHash: token.hash.value,
            userId: userId.value,
            expiresAt: token.expiresAt.value,
          })

          // Act
          const created = await repo.create(token).toPromise()

          // Assert
          expect(created).toBeDefined()
          expect(created).toEqual(token)
          expect(prisma.passwordResetToken.upsert).toHaveBeenCalledWith({
            where: { tokenHash: token.hash.value },
            create: {
              tokenHash: token.hash.value,
              userId: userId.value,
              expiresAt: token.expiresAt.value,
            },
            update: {},
          })
        })
      })
    })

    describe('given the user does not exist', () => {
      describe('when creating the token', () => {
        test('then it should raise an error', async () => {
          // Arrange
          const userId = UserId.random()
          const token = PasswordResetToken.create({
            token: Token.random(),
            userId,
          }).unwrap()

          prisma.passwordResetToken.upsert.mockRejectedValue(
            new Error('mocked error'),
          )

          // Act
          const created = repo.create(token)

          // Assert
          await expect(created.toPromise()).rejects.toThrowError('mocked error')
        })
      })
    })
  })

  describe('findByHash', () => {
    describe('given the token exists', () => {
      describe('when finding the token', () => {
        test('then it should find the token', async () => {
          // Arrange
          const token = PasswordResetToken.create({
            token: Token.random(),
            userId: UserId.random(),
          }).unwrap()

          prisma.passwordResetToken.findUniqueOrThrow.mockResolvedValue({
            tokenHash: token.hash.value,
            userId: token.userId.value,
            expiresAt: token.expiresAt.value,
          })

          // Act
          const found = await repo.findByHash(token.hash).toPromise()

          // Assert
          expect(found).toBeDefined()
          expect(found).toEqual(token)
          expect(
            prisma.passwordResetToken.findUniqueOrThrow,
          ).toHaveBeenCalledWith({
            where: { tokenHash: token.hash.value },
          })
        })
      })
    })

    describe("given the token doesn't exists", () => {
      describe('when finding the token', () => {
        test('then it should rise a not found error', async () => {
          // Arrange
          const token = PasswordResetToken.create({
            token: Token.random(),
            userId: UserId.random(),
          }).unwrap()

          prisma.passwordResetToken.findUniqueOrThrow.mockRejectedValue(
            new Error('mocked error'),
          )

          // Act
          const found = repo.findByHash(token.hash)

          // Assert
          await expect(found.toPromise()).rejects.toThrowError(/not found/i)
          expect(
            prisma.passwordResetToken.findUniqueOrThrow,
          ).toHaveBeenCalledWith({
            where: { tokenHash: token.hash.value },
          })
        })
      })
    })
  })

  describe('findByUserId', () => {
    describe('given the token exists', () => {
      describe('when finding the token', () => {
        test('then it should find the token', async () => {
          // Arrange
          const token = PasswordResetToken.create({
            token: Token.random(),
            userId: UserId.random(),
          }).unwrap()

          prisma.passwordResetToken.findFirst.mockResolvedValue({
            tokenHash: token.hash.value,
            userId: token.userId.value,
            expiresAt: token.expiresAt.value,
          })

          // Act
          const fetched = await repo.findByUserId(token.userId).toPromise()

          // Assert
          expect(fetched).toBeDefined()
          expect(fetched).toEqual(token)
          expect(prisma.passwordResetToken.findFirst).toHaveBeenCalledWith({
            where: { userId: token.userId.value },
          })
        })
      })
    })

    describe("given the token doesn't exists", () => {
      describe('when finding the token', () => {
        test('then it should throw a not found error', async () => {
          // Arrange
          const token = PasswordResetToken.create({
            token: Token.random(),
            userId: UserId.random(),
          }).unwrap()

          prisma.passwordResetToken.findFirst.mockResolvedValue(null)

          // Act
          const found = repo.findByUserId(token.userId)

          // Assert
          await expect(found.toPromise()).rejects.toThrow(/not found/i)
          expect(prisma.passwordResetToken.findFirst).toHaveBeenCalledWith({
            where: { userId: token.userId.value },
          })
        })
      })
    })
  })

  describe('deleteByUserId', () => {
    describe('given the token exists', () => {
      describe('when deleting the token', () => {
        test('then it should delete the token', async () => {
          // Arrange
          const userId = UserId.random()

          prisma.passwordResetToken.deleteMany.mockResolvedValue({
            count: 1,
          })

          // Act
          await repo.deleteByUserId(userId).toPromise()

          // Assert
          expect(prisma.passwordResetToken.deleteMany).toHaveBeenCalledWith({
            where: { userId: userId.value },
          })
        })
      })
    })

    describe("given the token doesn't exists", () => {
      describe('when deleting the token', () => {
        test('then it should not fail', async () => {
          // Arrange
          const userId = UserId.random()

          prisma.passwordResetToken.deleteMany.mockResolvedValue({
            count: 0,
          })

          // Act
          await repo.deleteByUserId(userId).toPromise()

          // Assert
          expect(prisma.passwordResetToken.deleteMany).toHaveBeenCalledWith({
            where: { userId: userId.value },
          })
        })
      })
    })
  })
})
