import { TestBed } from '@suites/unit'
import { beforeEach, describe, expect, test } from 'vitest'
import { LogIn } from './login.use-case.js'
import { faker } from '@faker-js/faker'
import { Mocked } from '@suites/doubles.vitest'
import {
  USER_REPOSITORY,
  UserRepository,
} from '#users/domain/repositories/user.repository.js'
import { User } from '#users/domain/entities/user.js'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { JwtService } from '@nestjs/jwt'
import { notFoundError, Task } from 'utils'
import { Email } from '#shared/entities/value-objects/email.js'

describe('LogIn', () => {
  let useCase: LogIn
  let repo: Mocked<UserRepository>
  let jwtService: Mocked<JwtService>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(LogIn).compile()

    useCase = unit

    repo = unitRef.get(USER_REPOSITORY) as unknown as Mocked<UserRepository>
    jwtService = unitRef.get(JwtService) as unknown as Mocked<JwtService>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given the input is valid', () => {
    let email: string
    let password: string

    beforeEach(async () => {
      email = faker.internet.email()
      password = faker.internet.password()
    })

    describe('when the credentials are valid', () => {
      let expectedUser: User
      let expectedToken: string

      beforeEach(() => {
        expectedUser = User.create({
          email,
          password: ValidatedPassword.validate(password).unwrap(),
          name: faker.person.fullName(),
        }).unwrap()
        expectedToken = faker.string.alphanumeric(10)

        repo.findByEmail.mockReturnValue(Task.of(expectedUser))
        jwtService.sign.mockReturnValue(expectedToken)
      })

      test('then it returns a token', async () => {
        const { token } = await useCase.execute({ email, password }).toPromise()
        expect(token).toBe(expectedToken)
        expect(repo.findByEmail).toHaveBeenCalledWith(new Email(email))
        expect(jwtService.sign).toHaveBeenCalledWith({
          sub: expectedUser.id.value,
          email,
        })
      })

      test('then it returns a user', async () => {
        const { user } = await useCase.execute({ email, password }).toPromise()
        expect(user).toEqual(expectedUser.toJSON())
        expect(repo.findByEmail).toHaveBeenCalledWith(new Email(email))
      })
    })

    describe("when the password doesn't match", () => {
      beforeEach(() => {
        repo.findByEmail.mockReturnValue(
          Task.of(
            User.create({
              email,
              password: ValidatedPassword.validate(
                faker.internet.password(),
              ).unwrap(),
              name: faker.person.fullName(),
            }).unwrap(),
          ),
        )
      })

      test('then it returns an unauthorized error', async () => {
        await expect(
          useCase.execute({ email, password }).toPromise(),
        ).rejects.toThrowError(/unauthorized/i)
      })
    })

    describe("when the user doesn't exist", () => {
      beforeEach(() => {
        repo.findByEmail.mockReturnValue(Task.reject(notFoundError()))
      })

      test('then it returns an unauthorized error', async () => {
        await expect(
          useCase.execute({ email, password }).toPromise(),
        ).rejects.toThrowError(/unauthorized/i)
      })
    })
  })

  describe('when the email is invalid', () => {
    test('then it returns an invalid email error', async () => {
      await expect(
        useCase
          .execute({
            email: faker.string.alphanumeric(10),
            password: faker.internet.password(),
          })
          .toPromise(),
      ).rejects.toThrowError(/invalid email/i)
    })
  })
})
