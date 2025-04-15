import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class CreateApiKeyInput {
  @Field({
    description: 'DApp ID',
  })
  dappId: string

  @Field({
    description: 'API key name',
  })
  name: string

  @Field({
    description: 'API key description',
    nullable: true,
  })
  description?: string
}
