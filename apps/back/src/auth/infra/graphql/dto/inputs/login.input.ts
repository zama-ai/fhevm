import { Field, InputType } from '@nestjs/graphql'

@InputType('LoginInput')
export class LoginInput {
  @Field()
  email: string

  @Field()
  password: string
}
