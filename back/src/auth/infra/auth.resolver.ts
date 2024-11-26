import { Args, Mutation, Resolver } from '@nestjs/graphql'
import { AuthType } from './dto/types/auth.type'
import { LoginInput } from './dto/inputs/login.input'
import { SignUp } from 'src/auth/use-cases/signup.use-case'
import { LogIn } from 'src/auth/use-cases/login.use-case'
import { SignupInput } from './dto/inputs/signup.input'

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
