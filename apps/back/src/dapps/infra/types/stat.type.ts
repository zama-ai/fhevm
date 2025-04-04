import { Field, ID, ObjectType, Int } from '@nestjs/graphql'

@ObjectType('CumulativeDappStats')
export class CumulativeDappStatsType {
  @Field(() => Int, { nullable: false })
  total: number

  @Field(() => Int, { nullable: false })
  FheAdd: number

  @Field(() => Int, { nullable: false })
  FheSub: number

  @Field(() => Int, { nullable: false })
  FheMul: number

  @Field(() => Int, { nullable: false })
  FheDiv: number

  @Field(() => Int, { nullable: false })
  FheRem: number

  @Field(() => Int, { nullable: false })
  FheBitAnd: number

  @Field(() => Int, { nullable: false })
  FheBitOr: number

  @Field(() => Int, { nullable: false })
  FheBitXor: number

  @Field(() => Int, { nullable: false })
  FheShl: number

  @Field(() => Int, { nullable: false })
  FheShr: number

  @Field(() => Int, { nullable: false })
  FheRotl: number

  @Field(() => Int, { nullable: false })
  FheRotr: number

  @Field(() => Int, { nullable: false })
  FheEq: number

  @Field(() => Int, { nullable: false })
  FheEqBytes: number

  @Field(() => Int, { nullable: false })
  FheNe: number

  @Field(() => Int, { nullable: false })
  FheNeBytes: number

  @Field(() => Int, { nullable: false })
  FheGe: number

  @Field(() => Int, { nullable: false })
  FheGt: number

  @Field(() => Int, { nullable: false })
  FheLe: number

  @Field(() => Int, { nullable: false })
  FheLt: number

  @Field(() => Int, { nullable: false })
  FheMin: number

  @Field(() => Int, { nullable: false })
  FheMax: number

  @Field(() => Int, { nullable: false })
  FheNeg: number

  @Field(() => Int, { nullable: false })
  FheNot: number

  @Field(() => Int, { nullable: false })
  VerifyCiphertext: number

  @Field(() => Int, { nullable: false })
  Cast: number

  @Field(() => Int, { nullable: false })
  TrivialEncrypt: number

  @Field(() => Int, { nullable: false })
  TrivialEncryptBytes: number

  @Field(() => Int, { nullable: false })
  FheIfThenElse: number

  @Field(() => Int, { nullable: false })
  FheRand: number

  @Field(() => Int, { nullable: false })
  FheRandBounded: number
}

@ObjectType('DailyDappStats')
export class DailyDappStatsType {
  @Field(() => ID, { nullable: false, description: 'The id of the day' })
  id: string

  @Field({ nullable: false })
  day: string

  @Field(() => Int, { nullable: false })
  total: number

  @Field(() => Int, { nullable: false })
  computation: number

  @Field(() => Int, { nullable: false })
  encryption: number
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
