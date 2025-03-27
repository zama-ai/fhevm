import { Field, ID, InputType } from '@nestjs/graphql'

@InputType('DeleteApiKeyInput')
export class DeleteApiKeyInput {
  @Field(() => ID, { nullable: false, description: 'API key ID' })
  id: string
}
