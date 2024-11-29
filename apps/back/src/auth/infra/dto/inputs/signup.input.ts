import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class SignupInput {
  @Field()
  password: string

  @Field()
  name: string

  @Field()
  invitationToken: string
}
