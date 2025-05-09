import { Field, InputType, Int } from '@nestjs/graphql'

@InputType()
export class ValidateAddressInput {
  @Field(() => Int, {
    nullable: false,
    description: '1 for eth mainnet, 11155111 for sepolia, etc',
  })
  chainId: number

  @Field({ nullable: false })
  address: string
}
