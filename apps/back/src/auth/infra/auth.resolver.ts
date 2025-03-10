import { Args, Mutation, Resolver } from '@nestjs/graphql'
import { AuthType } from './dto/types/auth.type.js'
import { LoginInput } from './dto/inputs/login.input.js'
import { SignUp } from '#auth/use-cases/signup.use-case.js'
import { LogIn } from '#auth/use-cases/login.use-case.js'
import { SignupInput } from './dto/inputs/signup.input.js'
import { UseFilters } from '@nestjs/common'
import { AppErrorFilter } from './filters/app-error.filter.js'

@UseFilters(AppErrorFilter)
@Resolver(() => AuthType)
export class AuthResolver {
  constructor(
    private readonly signupUC: SignUp,
    private readonly loginUC: LogIn,
  ) {}

  @Mutation(() => AuthType, { name: 'login' })
  login(@Args('input') input: LoginInput) {
    return this.loginUC.execute(input).toPromise()
  }

  @Mutation(() => AuthType, { name: 'signup' })
  signup(@Args('input') input: SignupInput) {
    return this.signupUC.execute(input).toPromise()
  }
}
