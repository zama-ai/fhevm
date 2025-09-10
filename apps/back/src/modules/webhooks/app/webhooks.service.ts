import { Injectable } from '@nestjs/common'
import { DeveloperPortalService } from './developer-portal.service.js'
import type { UserRegistered } from '../core/webhooks.types.js'
import { newOrganisationName } from '../core/user-registered.logic.js'

const DEFAULT_ORGANISATION_ID = 1;
@Injectable()
export class WebhookService {
  constructor(private readonly developerPortal: DeveloperPortalService) {}

  async handleUserRegistered(user: UserRegistered) {
    if (user.orgId === DEFAULT_ORGANISATION_ID) {
      // Create a new organisation
      const organisation = await this.developerPortal.createOrganisation(
        newOrganisationName(user),
      )

      // Assign the user to the new organisation, in the default team
      const teamIds = Array.isArray(organisation.Teams) && organisation.Teams.length > 0
        ? [organisation.Teams[0].ID]
        : [];
      await this.developerPortal.updateUser(user.id, {
        OrganisationID: organisation.ID,
        Teams: teamIds,
      })
    }
  }
}
