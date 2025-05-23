import { Field, InputType } from '@nestjs/graphql'

@InputType('requestResetPasswordInput', {
  description: 'Request reset password input',
})
export class RequestResetPasswordInput {
  @Field({ description: "The user's email address to reset the password" })
  email: string
}
