import { Field, InputType } from '@nestjs/graphql'

@InputType()
export class ValidateAddressInput {
  @Field({
    nullable: false,
    description: '"1" for eth mainnet "11155111" for sepolia, etc',
  })
  chainId: string

  @Field({ nullable: false })
  address: string
}
