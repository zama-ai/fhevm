import { Field, ID, InputType } from '@nestjs/graphql'

@InputType('UpdateApiKeyInput')
export class UpdateApiKeyInput {
  @Field(() => ID, { nullable: false, description: 'API key ID' })
  id: string

  @Field({ nullable: true, description: 'API key name' })
  name?: string

  @Field({ nullable: true, description: 'API key description' })
  description?: string
}
