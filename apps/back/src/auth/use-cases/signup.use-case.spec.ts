import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { SignUp, SignUpWithEmail, SignUpWithToken } from './signup.use-case.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'
import { TestBed } from '@suites/unit'
import { none, some, Task, UnitOfWork } from 'utils'
import { faker } from '@faker-js/faker'
import { User } from '#users/domain/entities/user.js'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { AddUserToTeam } from '#teams/use-cases/add-user-to-team.use-case.js'
import { CreateUser } from '#users/use-cases/create-user.use-case.js'
import { CreateTeam } from '#teams/use-cases/create-team.use-case.js'
import { Team } from '#teams/domain/entities/team.js'
import { PRODUCER, UNIT_OF_WORK } from '#constants.js'
import { GetUserByEmail } from '#users/use-cases/get-user-by-email.use-case.js'
import { IProducer } from '#shared/services/producer.js'
import {
  USER_TOKEN_REPOSITORY,
  UserTokenRepository,
} from '#auth/domain/repositories/user-token.repository.js'
import { UserToken } from '#auth/domain/entities/user-token.js'
import { Token } from '#auth/domain/entities/value-objects/token.js'

describe(SignUpWithEmail, () => {
  let useCase: SignUpWithEmail
  let getUserByEmailUC: Mocked<GetUserByEmail>
  let createUserUC: Mocked<CreateUser>
  let createTeamUC: Mocked<CreateTeam>
  let addUserToTeamUC: Mocked<AddUserToTeam>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(SignUpWithEmail).compile()

    useCase = unit

    let uow = unitRef.get(UNIT_OF_WORK) as unknown as Mocked<UnitOfWork>
    uow.exec.mockImplementation(task => task)

    getUserByEmailUC = unitRef.get(
      GetUserByEmail,
    ) as unknown as Mocked<GetUserByEmail>
    createUserUC = unitRef.get(CreateUser) as unknown as Mocked<CreateUser>
    createTeamUC = unitRef.get(CreateTeam) as unknown as Mocked<CreateTeam>
    addUserToTeamUC = unitRef.get(
      AddUserToTeam,
    ) as unknown as Mocked<AddUserToTeam>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given a user with the same email does not exist', () => {
    let email: string
    beforeEach(() => {
      email = faker.internet.email()
      getUserByEmailUC.execute.mockReturnValue(Task.of(none()))
    })

    describe('when the user sign up', () => {
      let password: string
      let name: string
      let task: ReturnType<SignUpWithEmail['execute']>

      let user: User
      let team: Team
      beforeEach(() => {
        password = faker.internet.password()
        name = faker.person.fullName()
        task = useCase.execute({ email, password, name })

        user = User.create({
          email,
          password: ValidatedPassword.validate(password).unwrap(),
          name,
        }).unwrap()
        team = Team.create({
          name: `${name}'s personal app`,
        }).unwrap()

        createUserUC.execute.mockReturnValue(Task.of(user))
        createTeamUC.execute.mockReturnValue(Task.of(team))
        addUserToTeamUC.execute.mockReturnValue(Task.of(team))
      })

      test('then it should create a new user', async () => {
        await task.toPromise()
        expect(createUserUC.execute).toHaveBeenCalledWith({
          email,
          password: ValidatedPassword.validate(password).unwrap(),
          name,
        })
      })

      test('then it should create a new team', async () => {
        await task.toPromise()
        expect(createTeamUC.execute).toHaveBeenCalledWith({ name: team.name })
      })

      test('then it should add the user to the team', async () => {
        await task.toPromise()
        expect(addUserToTeamUC.execute).toHaveBeenCalledWith({
          teamId: team.id,
          userId: user.id,
        })
      })
    })
  })

  describe('given a user with the same email already exists', () => {
    let email: string

    beforeEach(() => {
      email = faker.internet.email()
      let user = User.create({
        email,
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()
      getUserByEmailUC.execute.mockReturnValue(Task.of(some(user)))
    })

    describe('when the user sign up', () => {
      let password: string
      let name: string
      let task: ReturnType<SignUpWithEmail['execute']>

      beforeEach(() => {
        password = faker.internet.password()
        name = faker.person.fullName()
        task = useCase.execute({ email, password, name })
      })

      test('then it should return a email already in use error', async () => {
        await expect(task.toPromise()).rejects.toThrowError(
          /email already in use/i,
        )
      })

      test('then it should not create a new user', async () => {
        try {
          await task.toPromise()
        } catch {
          // ignore
        }
        expect(createUserUC.execute).not.toHaveBeenCalled()
      })

      test('then it should not create a new team', async () => {
        try {
          await task.toPromise()
        } catch {
          // ignore
        }
        expect(createTeamUC.execute).not.toHaveBeenCalled()
      })
    })
  })
})

describe(SignUpWithToken, () => {
  let useCase: SignUpWithToken
  let withEmail: Mocked<SignUpWithEmail>
  let repo: Mocked<UserTokenRepository>
  let producer: Mocked<IProducer>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(SignUpWithToken).compile()

    useCase = unit
    withEmail = unitRef.get(
      SignUpWithEmail,
    ) as unknown as Mocked<SignUpWithEmail>
    repo = unitRef.get(
      USER_TOKEN_REPOSITORY,
    ) as unknown as Mocked<UserTokenRepository>
    producer = unitRef.get(PRODUCER) as unknown as Mocked<IProducer>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('when signing up with email', () => {
    let email: string
    let password: string
    let name: string
    let user: User
    let token: Token

    beforeEach(() => {
      email = faker.internet.email()
      password = faker.internet.password()
      name = faker.person.fullName()
      user = User.create({
        email,
        password: ValidatedPassword.validate(password).unwrap(),
        name,
      }).unwrap()
      token = Token.random()
      withEmail.execute.mockReturnValue(Task.of({ user }))
      repo.create.mockReturnValue(
        Task.of(
          UserToken.create({
            token,
            type: 'CONFIRM_EMAIL',
            userId: user.id,
          }).unwrap(),
        ),
      )
      producer.publish.mockReturnValue(Task.of(void 0))
    })

    test('should call the withEmail', async () => {
      await useCase.execute({ email, password, name }).toPromise()
      expect(withEmail.execute).toHaveBeenCalledWith({ email, password, name })
    })

    test('should return the user', async () => {
      const { user: created } = await useCase
        .execute({ email, password, name })
        .toPromise()
      expect(created).toBe(user)
    })

    test('should create a token', async () => {
      await useCase.execute({ email, password, name }).toPromise()
      expect(repo.create).toHaveBeenCalledWith(token)
    })

    test('should publish a user:created event', async () => {
      await useCase.execute({ email, password, name }).toPromise()
      expect(producer.publish).toHaveBeenCalledWith(
        expect.objectContaining({
          type: 'back:user:created',
          payload: {
            requestId: expect.any(String),
            userId: user.id.value,
            email,
            name,
            token: expect.stringMatching(/^tkn_.*$/),
          },
        }),
      )
    })
  })
})

describe(SignUp, () => {
  let useCase: SignUp
  let featureFlagsService: Mocked<FeatureFlagsService>
  let withToken: Mocked<SignUpWithToken>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(SignUp).compile()

    useCase = unit
    featureFlagsService = unitRef.get(
      FEATURE_FLAGS_SERVICE,
    ) as unknown as Mocked<FeatureFlagsService>
    withToken = unitRef.get(
      SignUpWithToken,
    ) as unknown as Mocked<SignUpWithToken>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('when invitations is enabled', () => {
    beforeEach(() => {
      featureFlagsService.handle.mockImplementation(feature => {
        if (feature === 'INVITATIONS') {
          return Task.of(true)
        }
        return Task.of(false)
      })
    })

    test('should rejects due invitations been enabled', async () => {
      let email = faker.internet.email()
      let name = faker.person.fullName()
      let password = faker.internet.password()

      await expect(
        useCase
          .execute({
            email,
            name,
            password,
          })
          .toPromise(),
      ).rejects.toThrowError(/invitations are enabled/i)
    })

    test('should not call the withToken', async () => {
      try {
        await useCase
          .execute({
            email: faker.internet.email(),
            name: faker.person.fullName(),
            password: faker.internet.password(),
          })
          .toPromise()
      } catch {}
      expect(withToken.execute).not.toBeCalled()
    })
  })

  describe('when invitations is disabled', () => {
    let user: User

    beforeEach(() => {
      featureFlagsService.handle.mockImplementation(feature => {
        if (feature === 'INVITATIONS') {
          return Task.of(false)
        }
        return Task.of(false)
      })

      user = User.create({
        email: faker.internet.email(),
        password: ValidatedPassword.validate(
          faker.internet.password(),
        ).unwrap(),
        name: faker.person.fullName(),
      }).unwrap()

      withToken.execute.mockReturnValue(Task.of({ user }))
    })

    test('should call the withToken', async () => {
      let email = faker.internet.email()
      let name = faker.person.fullName()
      let password = faker.internet.password()

      await useCase.execute({ email, name, password }).toPromise()

      expect(withToken.execute).toHaveBeenCalledExactlyOnceWith({
        email,
        name,
        password,
      })
    })

    test('should return the right result', async () => {
      const result = await useCase
        .execute({
          email: faker.internet.email(),
          name: faker.person.fullName(),
          password: faker.internet.password(),
        })
        .toPromise()
      expect(result).toEqual({ user })
    })
  })
})
