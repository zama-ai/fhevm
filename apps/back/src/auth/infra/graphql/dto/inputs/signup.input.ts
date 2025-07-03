import { Field, InputType } from '@nestjs/graphql'

@InputType('SignupInput')
export class SignupInput {
  @Field()
  password: string

  @Field()
  name: string

  @Field()
  email: string
}
