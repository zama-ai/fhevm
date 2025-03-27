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
