import { Field, ID, InputType } from '@nestjs/graphql'

@InputType()
export class UpdateUserInput {
  @Field(() => ID, { nullable: false })
  id: `user_${string}`

  @Field({ nullable: false })
  name: string
}
