import { Field, InputType } from '@nestjs/graphql'

@InputType('resetPasswordInput')
export class ResetPasswordInput {
  @Field({ description: 'The token received by email' })
  token: string

  @Field({ description: 'The new password' })
  password: string
}
