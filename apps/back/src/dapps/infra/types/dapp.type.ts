import { TeamType } from '#users/infra/types/team.type.js'
import { Field, ID, ObjectType, registerEnumType } from '@nestjs/graphql'

export enum DappStatus {
  DRAFT = 'DRAFT',
  DEPLOYING = 'DEPLOYING',
  LIVE = 'LIVE',
  FAILED = 'FAILED',
  DELETED = 'DELETED',
  ARCHIVED = 'ARCHIVED',
}

registerEnumType(DappStatus, {
  name: 'DappStatus',
  valuesMap: {
    DRAFT: {
      description: 'Still being worked on',
    },
    DEPLOYING: {
      description: 'We are deploying it',
    },
    LIVE: {
      description: 'You can use it now',
    },
    DELETED: {
      deprecationReason: 'Not implmented yet',
    },
  },
})

@ObjectType('Stats')
export class StatsType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  name: string

  @Field({ nullable: false })
  timestamp: number

  @Field({ nullable: false })
  externalRef: string
}

@ObjectType('Dapp')
export class DappType {
  @Field(() => ID, { nullable: false })
  id: string

  @Field({ nullable: false })
  name: string

  @Field(() => DappStatus, { nullable: false })
  status: DappStatus

  @Field({ nullable: true })
  address: string

  // TODO: Discuss wether this should be kept or not
  // pro: we can use it to resolve the teams on dapps.resolver
  // pro: it could be used in frontend to cheaply check the team
  // con: it is a bit of a hack
  // con: it is not standard
  @Field({
    nullable: false,
    deprecationReason:
      'Do not use this, it shall go away when I find a way to make it disappear',
  })
  teamId: string

  @Field(() => TeamType, { nullable: false })
  team: TeamType

  @Field({ nullable: false })
  createdAt: number

  @Field(() => [StatsType], {
    nullable: false,
    description: 'DApp usage statistics',
  })
  stats: StatsType[]
}
