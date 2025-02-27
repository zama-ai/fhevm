import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class ValidateAddressInput {
  @Field({ nullable: false })
  chainId: string

  @Field({ nullable: false })
  address: string
}
