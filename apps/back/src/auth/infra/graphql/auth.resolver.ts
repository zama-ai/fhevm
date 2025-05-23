import { Args, Mutation, Resolver } from '@nestjs/graphql'
import { AuthType } from './dto/types/auth.type.js'
import { LoginInput } from './dto/inputs/login.input.js'
import { SignupInput } from './dto/inputs/signup.input.js'
import { Inject, UseFilters } from '@nestjs/common'
import { AppErrorFilter } from '../filters/app-error.filter.js'
import { RequestResetPasswordInput } from './dto/inputs/request-reset-password.input.js'
import * as uc from '#auth/use-cases/index.js'
import { ResetPasswordInput } from './dto/inputs/reset-password.input.js'

@UseFilters(AppErrorFilter)
@Resolver(() => AuthType)
export class AuthResolver {
  constructor(
    private readonly signupUC: uc.SignUp,
    private readonly loginUC: uc.LogIn,
    private readonly createResetPasswordTokenUC: uc.CreateResetPasswordToken,
    @Inject(uc.RESET_PASSWORD)
    private readonly resetPasswordUC: uc.IResetPassword,
  ) {}

  @Mutation(() => AuthType, { name: 'login' })
  login(@Args('input') input: LoginInput) {
    return this.loginUC.execute(input).toPromise()
  }

  @Mutation(() => AuthType, { name: 'signup' })
  signup(@Args('input') input: SignupInput) {
    return this.signupUC.execute(input).toPromise()
  }

  @Mutation(() => Boolean, { name: 'requestResetPassword' })
  async requestResetPassword(@Args('input') input: RequestResetPasswordInput) {
    await this.createResetPasswordTokenUC.execute(input).toPromise()
    return true
  }

  @Mutation(() => AuthType, { name: 'resetPassword' })
  resetPassword(@Args('input') input: ResetPasswordInput) {
    return this.resetPasswordUC.execute(input).toPromise()
  }
}
