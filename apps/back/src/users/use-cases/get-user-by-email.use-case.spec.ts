import { User } from '#users/domain/entities/user.js'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import {
  USER_REPOSITORY,
  UserRepository,
} from '#users/domain/repositories/user.repository.js'
import { faker } from '@faker-js/faker'
import { Mocked } from '@suites/doubles.vitest'
import { TestBed } from '@suites/unit'
import { AppError, notFoundError, Task } from 'utils'
import { beforeEach, describe, expect, test } from 'vitest'
import { GetUserByEmail } from './get-user-by-email.use-case.js'
import { Email } from '#shared/entities/value-objects/email.js'

describe('GetUserByEmail', () => {
  let useCase: GetUserByEmail
  let repo: Mocked<UserRepository>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(GetUserByEmail).compile()

    useCase = unit
    repo = unitRef.get(USER_REPOSITORY) as unknown as Mocked<UserRepository>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given the user exists', () => {
    let email: string
    let user: User

    beforeEach(() => {
      email = faker.internet.email()
      user = User.create({
        email,
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()

      repo.findByEmail.mockReturnValue(Task.of(user))
    })

    describe('when called', () => {
      let result: Task<User, AppError>

      beforeEach(() => {
        result = useCase.execute(email)
      })

      test('then it returns the user', async () => {
        const expected = await result.toPromise()
        expect(expected).toEqual(user)
      })

      test('then it calls the repo', async () => {
        await result.toPromise()
        expect(repo.findByEmail).toHaveBeenCalledWith(new Email(email))
      })
    })
  })

  describe('given the user does not exist', () => {
    let email: string

    beforeEach(() => {
      email = faker.internet.email()

      repo.findByEmail.mockReturnValue(Task.reject(notFoundError()))
    })

    describe('when called', () => {
      let result: Task<User, AppError>

      beforeEach(() => {
        result = useCase.execute(email)
      })

      test('then it returns a not found error', async () => {
        await expect(result.toPromise()).rejects.toThrowError(/not found/i)
      })
    })
  })
})
