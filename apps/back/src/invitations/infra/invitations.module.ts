import { DynamicModule, Module } from '@nestjs/common'
import { InvitationsResolver } from './graphql/invitations.resolver.js'
import { DatabaseModule } from '#infra/database/database.module.js'
import { GetInvitationByToken } from '#invitations/use-cases/get-invitation-by-token.use-case.js'
import { CreateInvitation } from '#invitations/use-cases/create-invitation.use-case.js'
import { INVITATION_REPOSITORY } from '#invitations/domain/repositories/invitation.repository.js'
import { PrismaInvitationRepository } from './db/prisma-invitation.repository.js'
import { MarkInvitationAsUsed } from '#invitations/use-cases/mark-invitation-as-used.use-case.js'
import {
  FEATURE_FLAGS_SERVICE,
  FeatureFlagHandler,
} from '#feature-flag/services/feature-flags.service.js'

@Module({
  imports: [DatabaseModule],
  providers: [
    GetInvitationByToken,
    MarkInvitationAsUsed,
    CreateInvitation,
    InvitationsResolver,
    {
      provide: INVITATION_REPOSITORY,
      useClass: PrismaInvitationRepository,
    },
  ],
  exports: [GetInvitationByToken, MarkInvitationAsUsed],
})
export class InvitationsModule {}
