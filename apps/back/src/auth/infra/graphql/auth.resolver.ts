import { Args, Mutation, Resolver } from '@nestjs/graphql'
import { AuthType } from './dto/types/auth.type.js'
import { LoginInput } from './dto/inputs/login.input.js'
import { Inject, Logger, UseFilters } from '@nestjs/common'
import { AppErrorFilter } from '../filters/app-error.filter.js'
import { RequestResetPasswordInput } from './dto/inputs/request-reset-password.input.js'
import * as uc from '#auth/use-cases/index.js'
import { ResetPasswordInput } from './dto/inputs/reset-password.input.js'
import { SignupInput } from './dto/inputs/signup.input.js'
import { SignupWithInvitationTokenInput } from './dto/inputs/signup-with-invitation-token.input.js'
import { AuthWithTokenType } from './dto/types/auth-with-token.type.js'
import { ConfirmEmailInput } from './dto/inputs/confirm-email.input.js'

@UseFilters(AppErrorFilter)
// @Resolver(() => AuthType)
@Resolver()
export class AuthResolver {
  private readonly logger = new Logger(AuthResolver.name)

  @Inject(uc.SIGN_UP)
  private readonly signupUC: uc.ISignUp

  @Inject(uc.CONFIRM_EMAIL)
  private readonly confirmEmailUC: uc.IConfirmEmail

  @Inject(uc.SIGN_UP_WITH_INVITATION_TOKEN)
  private readonly signupWithInvitationTokenUC: uc.ISignUpWithInvitationToken

  @Inject(uc.LOG_IN)
  private readonly loginUC: uc.ILogIn

  @Inject(uc.CREATE_RESET_PASSWORD_TOKEN)
  private readonly createResetPasswordTokenUC: uc.ICreateResetPasswordToken

  @Inject(uc.RESET_PASSWORD)
  private readonly resetPasswordUC: uc.IResetPassword

  @Mutation(() => AuthWithTokenType, { name: 'login' })
  async login(@Args('input') input: LoginInput) {
    try {
      const { user, token } = await this.loginUC.execute(input).toPromise()
      return {
        user: user.toJSON(),
        token,
      }
    } catch (err) {
      this.logger.warn(`login failed: ${err}`)
      throw err
    }
  }

  @Mutation(() => AuthWithTokenType, { name: 'signupWithInvitation' })
  async signupWithInvitation(
    @Args('input') input: SignupWithInvitationTokenInput,
  ) {
    try {
      const { user, token } = await this.signupWithInvitationTokenUC
        .execute(input)
        .toPromise()
      return {
        user: user.toJSON(),
        token,
      }
    } catch (e) {
      this.logger.warn(`signup with invitation failed: ${e}`)
      throw e
    }
  }

  @Mutation(() => AuthType, { name: 'signup' })
  async signup(@Args('input') input: SignupInput) {
    try {
      const { user } = await this.signupUC.execute(input).toPromise()
      return {
        user: user.toJSON(),
      }
    } catch (e) {
      this.logger.warn(`signup failed: ${e}`)
      throw e
    }
  }

  @Mutation(() => AuthWithTokenType, { name: 'confirmEmail' })
  async confirmEmail(@Args('input') input: ConfirmEmailInput) {
    try {
      const { user, token: newToken } = await this.confirmEmailUC
        .execute(input)
        .toPromise()
      return {
        user: user.toJSON(),
        token: newToken,
      }
    } catch (e) {
      this.logger.warn(`confirm email failed: ${e}`)
      throw e
    }
  }

  @Mutation(() => Boolean, { name: 'requestResetPassword' })
  async requestResetPassword(@Args('input') input: RequestResetPasswordInput) {
    try {
      await this.createResetPasswordTokenUC.execute(input).toPromise()
      return true
    } catch (err) {
      this.logger.warn(`request reset password failed: ${err}`)
      throw err
    }
  }

  @Mutation(() => AuthWithTokenType, { name: 'resetPassword' })
  async resetPassword(@Args('input') input: ResetPasswordInput) {
    try {
      const { user, token } = await this.resetPasswordUC
        .execute(input)
        .toPromise()
      return { user: user.toJSON(), token }
    } catch (err) {
      this.logger.warn(`reset password failed: ${err}`)
      throw err
    }
  }
}
