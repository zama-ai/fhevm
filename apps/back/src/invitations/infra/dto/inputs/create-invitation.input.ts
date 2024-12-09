import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class CreateInvitationInput {
  @Field()
  email: string

  /**
   * This secret allows zama to control who can create an account
   */
  @Field({
    description:
      'You need the secret key to create an invitation ask the #zws team to get one',
  })
  secret: string
}
