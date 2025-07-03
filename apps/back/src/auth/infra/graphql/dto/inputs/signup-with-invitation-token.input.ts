import { Field, InputType } from '@nestjs/graphql'

@InputType('SignupWithInvitationTokenInput')
export class SignupWithInvitationTokenInput {
  @Field()
  password: string

  @Field()
  name: string

  @Field()
  invitationToken: string
}
