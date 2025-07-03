import { beforeEach, describe, expect, Mocked, test } from 'vitest'
import { SignUpWithInvitationToken } from './signup-with-invitation-token.use-case.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagsService,
} from '#feature-flag/services/feature-flags.service.js'
import { TestBed } from '@suites/unit'
import { none, notFoundError, some, Task, UnitOfWork } from 'utils'
import { faker } from '@faker-js/faker'
import { User } from '#users/domain/entities/user.js'
import { ValidatedPassword } from '#users/domain/entities/value-objects.js'
import { GetInvitationByToken } from '#invitations/use-cases/get-invitation-by-token.use-case.js'
import { MarkInvitationAsUsed } from '#invitations/use-cases/mark-invitation-as-used.use-case.js'
import { AddUserToTeam } from '#teams/use-cases/add-user-to-team.use-case.js'
import { CreateUser } from '#users/use-cases/create-user.use-case.js'
import { ConfirmUser } from '#users/use-cases/confirm-user.use-case.js'
import { type ILogIn, LOG_IN } from './login.use-case.js'
import { Invitation } from '#invitations/domain/entities/invitation.js'
import { CreateTeam } from '#teams/use-cases/create-team.use-case.js'
import { Team } from '#teams/domain/entities/team.js'
import { UNIT_OF_WORK } from '#constants.js'
import { Token } from '#invitations/domain/entities/value-objects.js'

describe(SignUpWithInvitationToken, () => {
  let useCase: SignUpWithInvitationToken
  let getInvitationByTokenUC: Mocked<GetInvitationByToken>
  let markInvitationAsUsedUC: Mocked<MarkInvitationAsUsed>
  let createUserUC: Mocked<CreateUser>
  let createTeamUC: Mocked<CreateTeam>
  let confirmUserUC: Mocked<ConfirmUser>
  let addUserToTeamUC: Mocked<AddUserToTeam>
  let loginUc: Mocked<ILogIn>

  beforeEach(async () => {
    const { unit, unitRef } = await TestBed.solitary(
      SignUpWithInvitationToken,
    ).compile()

    useCase = unit

    let uow = unitRef.get(UNIT_OF_WORK) as unknown as Mocked<UnitOfWork>
    uow.exec.mockImplementation(task => task)

    getInvitationByTokenUC = unitRef.get(
      GetInvitationByToken,
    ) as unknown as Mocked<GetInvitationByToken>
    markInvitationAsUsedUC = unitRef.get(
      MarkInvitationAsUsed,
    ) as unknown as Mocked<MarkInvitationAsUsed>
    createUserUC = unitRef.get(CreateUser) as unknown as Mocked<CreateUser>
    createTeamUC = unitRef.get(CreateTeam) as unknown as Mocked<CreateTeam>
    confirmUserUC = unitRef.get(ConfirmUser) as unknown as Mocked<ConfirmUser>
    addUserToTeamUC = unitRef.get(
      AddUserToTeam,
    ) as unknown as Mocked<AddUserToTeam>
    loginUc = unitRef.get(LOG_IN) as unknown as Mocked<ILogIn>
  })

  test('should be defined', () => {
    expect(useCase).toBeDefined()
  })

  describe('given a valid invitation token', () => {
    let email: string
    let invitation: Invitation
    beforeEach(() => {
      email = faker.internet.email()
      invitation = Invitation.create({ email }).unwrap()
      getInvitationByTokenUC.execute.mockReturnValue(Task.of(invitation))
    })

    describe('when the user sign up', () => {
      let password: string
      let name: string

      let task: ReturnType<SignUpWithInvitationToken['execute']>
      let user: User
      let team: Team
      let token: string

      beforeEach(() => {
        password = faker.internet.password()
        name = faker.person.fullName()
        user = User.create({
          email: invitation.email,
          password: ValidatedPassword.validate(password).unwrap(),
          name,
        }).unwrap()
        team = Team.create({
          name: `${name}'s personal app`,
        }).unwrap()
        token = faker.string.uuid()

        markInvitationAsUsedUC.execute.mockReturnValue(Task.of(invitation))
        createUserUC.execute.mockReturnValue(Task.of(user))
        createTeamUC.execute.mockReturnValue(Task.of(team))
        confirmUserUC.execute.mockReturnValue(Task.of(user))
        addUserToTeamUC.execute.mockReturnValue(Task.of(team))
        loginUc.execute.mockReturnValue(Task.of({ user, token }))

        task = useCase.execute({
          invitationToken: invitation.token.value,
          name,
          password,
        })
      })

      test('then it should mark the invitation as used', async () => {
        await task.toPromise()
        expect(markInvitationAsUsedUC.execute).toHaveBeenCalledWith(invitation)
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

      test('then it should confirm the user', async () => {
        await task.toPromise()
        expect(confirmUserUC.execute).toHaveBeenCalledWith({ id: user.id })
      })

      test('then it should log in the user', async () => {
        await task.toPromise()
        expect(loginUc.execute).toHaveBeenCalledWith({ email, password })
      })
    })
  })

  describe('given an invalid invitation token', () => {
    let token: string

    beforeEach(() => {
      token = Token.random().value
      getInvitationByTokenUC.execute.mockReturnValue(
        Task.reject(notFoundError()),
      )
    })

    describe('when the user sign up', () => {
      let task: ReturnType<SignUpWithInvitationToken['execute']>

      beforeEach(() => {
        task = useCase.execute({
          invitationToken: token,
          name: faker.person.fullName(),
          password: faker.internet.password(),
        })
      })

      test('then it should return a not found error', async () => {
        await expect(task.toPromise()).rejects.toThrowError(/not found/i)
      })

      test('then it should not create a user', async () => {
        try {
          await task.toPromise()
        } catch {
          // ignore
        }
        expect(createUserUC.execute).not.toHaveBeenCalled()
      })

      test('then it should not create a team', async () => {
        try {
          await task.toPromise()
        } catch {
          // ignore
        }
        expect(createTeamUC.execute).not.toHaveBeenCalled()
      })

      test('then it should not mark the invitation as used', async () => {
        try {
          await task.toPromise()
        } catch {
          // ignore
        }
        expect(markInvitationAsUsedUC.execute).not.toHaveBeenCalled()
      })

      test('then it should not log in the user', async () => {
        try {
          await task.toPromise()
        } catch {
          // ignore
        }
        expect(loginUc.execute).not.toHaveBeenCalled()
      })
    })
  })
})
