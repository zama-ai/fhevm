import { Args, Mutation, Query, Resolver } from '@nestjs/graphql'
import { CreateInvitationInput } from '#invitations/infra/dto/inputs/create-invitation.input.js'
import { GetInvitationByToken } from '#invitations/use-cases/get-invitation-by-token.use-case.js'
import { CreateInvitation } from '#invitations/use-cases/create-invitation.use-case.js'
import { InvitationType } from './types/invitations.type.js'

@Resolver(() => InvitationType)
export class InvitationsResolver {
  constructor(
    private readonly getInvitationByTokenUC: GetInvitationByToken,
    private readonly createInvitationUC: CreateInvitation,
  ) {}

  @Query(() => InvitationType, { name: 'invitation' })
  invitation(@Args('token') token: string) {
    return this.getInvitationByTokenUC.execute(token).toPromise()
  }

  @Mutation(() => InvitationType, { name: 'createInvitation' })
  createInvitation(@Args('input') input: CreateInvitationInput) {
    return this.createInvitationUC.execute(input).toPromise()
  }
}
