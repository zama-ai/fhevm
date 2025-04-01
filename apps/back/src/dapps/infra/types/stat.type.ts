import { Field, ID, ObjectType, Int } from '@nestjs/graphql'

// @ObjectType('DailyDappStats')
// export class DailyDappStatsType {
//   @Field(() => ID, { nullable: false, description: 'The id of the day' })
//   id: string

//   @Field({ nullable: false })
//   day: string

//   @Field({ nullable: false })
//   total: number

//   @Field({ nullable: false })
//   FheAdd: number
// }

@ObjectType('CumulativeDappStats')
export class CumulativeDappStatsType {
  @Field(() => Int, { nullable: false })
  total: number

  @Field(() => Int, { nullable: false })
  FheAdd: number

  @Field(() => Int, { nullable: false })
  FheBitAnd: number

  @Field(() => Int, { nullable: false })
  FheIfThenElse: number

  @Field(() => Int, { nullable: false })
  FheLe: number

  @Field(() => Int, { nullable: false })
  FheOr: number

  @Field(() => Int, { nullable: false })
  FheSub: number

  @Field(() => Int, { nullable: false })
  TrivialEncrypt: number

  @Field(() => Int, { nullable: false })
  VerifyCiphertext: number

  @Field(() => Int, { nullable: false })
  FheMul: number

  @Field(() => Int, { nullable: false })
  FheDiv: number
}

@ObjectType('DappStats')
export class DappStatsType {
  @Field(() => ID, { nullable: false, description: 'The id of the dapp' })
  id: string

  // @Field(() => CumulativeDappStatsType, { nullable: false })
  // cumulative: CumulativeDappStatsType

  //   @Field({ nullable: false })
  //   byDay: DailyDappStatsType[]
}
