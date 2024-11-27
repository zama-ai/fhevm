import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class LoginInput {
  @Field()
  email: string

  @Field()
  password: string
}
