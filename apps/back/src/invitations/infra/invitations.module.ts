import { Module } from '@nestjs/common'
import { InvitationsResolver } from './graphql/invitations.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import { GetInvitationByToken } from '#invitations/use-cases/get-invitation-by-token.use-case.js'
import { CreateInvitation } from '#invitations/use-cases/create-invitation.use-case.js'
import { INVITATION_REPOSITORY } from '#invitations/domain/repositories/invitation.repository.js'
import { PrismaInvitationRepository } from './db/prisma-invitation.repository.js'
import { MarkInvitationAsUsed } from '#invitations/use-cases/mark-invitation-as-used.use-case.js'

@Module({
  imports: [DatabaseModule],
  providers: [
    InvitationsResolver,
    CreateInvitation,
    GetInvitationByToken,
    MarkInvitationAsUsed,
    {
      provide: INVITATION_REPOSITORY,
      useClass: PrismaInvitationRepository,
    },
  ],
  exports: [GetInvitationByToken, MarkInvitationAsUsed],
})
export class InvitationsModule {}
