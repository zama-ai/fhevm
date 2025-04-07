import { Field, ID, ObjectType } from '@nestjs/graphql'

@ObjectType('ApiKey')
export class ApiKeyType {
  @Field(() => ID, { nullable: false, description: 'API key ID' })
  id: string

  @Field(() => ID, { nullable: false, description: 'DApp ID' })
  dappId: string

  @Field(() => String, { nullable: false, description: 'API key name' })
  name: string

  @Field(() => String, { nullable: true, description: 'API key description' })
  description?: string | null
}

@ObjectType('CreateApiKey')
export class CreateApiKeyType {
  @Field(() => String, {
    nullable: false,
    description: 'API Key token to use for authentication',
  })
  token: string

  @Field(() => ApiKeyType, { nullable: false, description: 'API Key details' })
  apiKey: ApiKeyType
}
