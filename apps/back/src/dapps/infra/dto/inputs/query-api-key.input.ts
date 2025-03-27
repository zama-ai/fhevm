import { Field, ID, InputType } from '@nestjs/graphql'

@InputType()
export class QueryApiKeyInput {
  @Field(() => ID, { nullable: false, description: 'API key ID' })
  id: string
}
