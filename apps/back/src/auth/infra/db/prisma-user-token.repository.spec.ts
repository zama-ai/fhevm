import { UserTokenRepository } from '#auth/domain/repositories/user-token.repository.js'
import { beforeEach, describe, expect, test } from 'vitest'
import { PrismaUserTokenRepository } from './prisma-user-token.repository.js'
import { PrismaService } from '#infra/database/prisma.service.js'
import { Mocked } from '@suites/doubles.vitest'
import { TestBed } from '@suites/unit'
import { UserId } from '#users/domain/entities/value-objects.js'
import { UserToken, UserTokenTypes } from '#auth/domain/entities/user-token.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'
import { faker } from '@faker-js/faker'

describe('PrismaUserTokenRepository', () => {
  let repo: UserTokenRepository
  let prisma: Mocked<PrismaService>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      PrismaUserTokenRepository,
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
          const token = UserToken.create({
            token: Token.random(),
            userId,
            type: 'RESET_PASSWORD',
          }).unwrap()

          prisma.userToken.upsert.mockResolvedValue({
            tokenHash: token.hash.value,
            userId: userId.value,
            expiresAt: token.expiresAt.value,
            type: 'RESET_PASSWORD',
          })

          // Act
          const created = await repo.create(token).toPromise()

          // Assert
          expect(created).toBeDefined()
          expect(created).toEqual(token)
          expect(prisma.userToken.upsert).toHaveBeenCalledWith({
            where: { tokenHash: token.hash.value },
            create: {
              tokenHash: token.hash.value,
              userId: userId.value,
              expiresAt: token.expiresAt.value,
              type: 'RESET_PASSWORD',
            },
            update: {},
          })
        })
        test('then it should create a confirm email token', async () => {
          // Arrange
          const userId = UserId.random()
          const token = UserToken.create({
            token: Token.random(),
            userId,
            type: 'CONFIRM_EMAIL',
          }).unwrap()

          prisma.userToken.upsert.mockResolvedValue({
            tokenHash: token.hash.value,
            userId: userId.value,
            expiresAt: token.expiresAt.value,
            type: 'CONFIRM_EMAIL',
          })

          // Act
          const created = await repo.create(token).toPromise()

          // Assert
          expect(created).toBeDefined()
          expect(created).toEqual(token)
          expect(prisma.userToken.upsert).toHaveBeenCalledWith({
            where: { tokenHash: token.hash.value },
            create: {
              tokenHash: token.hash.value,
              userId: userId.value,
              expiresAt: token.expiresAt.value,
              type: 'CONFIRM_EMAIL',
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
          const token = UserToken.create({
            token: Token.random(),
            userId,
            type: faker.helpers.arrayElement(UserTokenTypes),
          }).unwrap()

          prisma.userToken.upsert.mockRejectedValue(new Error('mocked error'))

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
          const token = UserToken.create({
            token: Token.random(),
            userId: UserId.random(),
            type: faker.helpers.arrayElement(UserTokenTypes),
          }).unwrap()

          prisma.userToken.findUniqueOrThrow.mockResolvedValue({
            tokenHash: token.hash.value,
            userId: token.userId.value,
            expiresAt: token.expiresAt.value,
            type: token.type,
          })

          // Act
          const found = await repo.findByHash(token.hash).toPromise()

          // Assert
          expect(found).toBeDefined()
          expect(found).toEqual(token)
          expect(prisma.userToken.findUniqueOrThrow).toHaveBeenCalledWith({
            where: { tokenHash: token.hash.value },
          })
        })
      })
    })

    describe("given the token doesn't exists", () => {
      describe('when finding the token', () => {
        test('then it should rise a not found error', async () => {
          // Arrange
          const token = UserToken.create({
            token: Token.random(),
            userId: UserId.random(),
            type: faker.helpers.arrayElement(UserTokenTypes),
          }).unwrap()

          prisma.userToken.findUniqueOrThrow.mockRejectedValue(
            new Error('mocked error'),
          )

          // Act
          const found = repo.findByHash(token.hash)

          // Assert
          await expect(found.toPromise()).rejects.toThrowError(/not found/i)
          expect(prisma.userToken.findUniqueOrThrow).toHaveBeenCalledWith({
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
          const token = UserToken.create({
            token: Token.random(),
            userId: UserId.random(),
            type: faker.helpers.arrayElement(UserTokenTypes),
          }).unwrap()

          prisma.userToken.findFirst.mockResolvedValue({
            tokenHash: token.hash.value,
            userId: token.userId.value,
            expiresAt: token.expiresAt.value,
            type: token.type,
          })

          // Act
          const fetched = await repo
            .findByUserId(token.userId, token.type)
            .toPromise()

          // Assert
          expect(fetched).toBeDefined()
          expect(fetched).toEqual(token)
          expect(prisma.userToken.findFirst).toHaveBeenCalledWith({
            where: { userId: token.userId.value, type: token.type },
          })
        })
      })
    })

    describe("given the token doesn't exists", () => {
      describe('when finding the token', () => {
        test('then it should throw a not found error', async () => {
          // Arrange
          const token = UserToken.create({
            token: Token.random(),
            userId: UserId.random(),
            type: faker.helpers.arrayElement(UserTokenTypes),
          }).unwrap()

          prisma.userToken.findFirst.mockResolvedValue(null)

          // Act
          const found = repo.findByUserId(token.userId, token.type)

          // Assert
          await expect(found.toPromise()).rejects.toThrow(/not found/i)
          expect(prisma.userToken.findFirst).toHaveBeenCalledWith({
            where: { userId: token.userId.value, type: token.type },
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

          prisma.userToken.deleteMany.mockResolvedValue({
            count: 1,
          })

          // Act
          await repo.deleteByUserId(userId).toPromise()

          // Assert
          expect(prisma.userToken.deleteMany).toHaveBeenCalledWith({
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

          prisma.userToken.deleteMany.mockResolvedValue({
            count: 0,
          })

          // Act
          await repo.deleteByUserId(userId).toPromise()

          // Assert
          expect(prisma.userToken.deleteMany).toHaveBeenCalledWith({
            where: { userId: userId.value },
          })
        })
      })
    })
  })
})
