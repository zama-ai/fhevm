import { Injectable, Logger } from '@nestjs/common'

import * as tyk from './tyk.types.js'
import { UserRegisteredService } from '../core/user-registered.machine.js'
import type * as core from '../core/types.js'
import { ApplicationRegisteredService } from '../core/application-registered.machine.js'
import { HttpService } from '@nestjs/axios'
import { firstValueFrom } from 'rxjs'

@Injectable()
export class DeveloperPortalService
  implements UserRegisteredService, ApplicationRegisteredService
{
  private readonly logger = new Logger(DeveloperPortalService.name)
  constructor(private readonly http: HttpService) {}

  createOrganisation = async (
    organisationName: string,
  ): Promise<core.NewOrganisation> => {
    const payload = { Name: organisationName }
    this.logger.verbose(`creating org: ${JSON.stringify(payload)}`)

    const response = await firstValueFrom(
      this.http.post<tyk.NewOrganizationResponse>('/organisations', payload),
    )

    const org = response.data

    this.logger.verbose(`create org response: ${JSON.stringify(org)}`)
    return {
      id: org.ID,
      name: org.Name,
      teams: org.Teams.map(team => ({
        id: team.ID,
        name: team.Name,
        default: team.Default,
      })),
    } satisfies core.NewOrganisation
  }

  updateUser = async (
    userId: number,
    payload: Partial<Omit<core.User, 'id'>>,
  ): Promise<core.User> => {
    this.logger.verbose(`updating user ${userId}: ${JSON.stringify(payload)}`)
    const { data: user } = await firstValueFrom(
      this.http.put<tyk.User, Partial<Omit<tyk.User, 'id'>>>(
        `/users/${userId}`,
        {
          ...(payload.email && { Email: payload.email }),
          ...(payload.orgId && { OrganisationID: payload.orgId }),
          ...(payload.teamIds && { Teams: payload.teamIds }),
        },
      ),
    )
    this.logger.verbose(`update user response: ${JSON.stringify(user)}`)
    return {
      id: user.ID,
      email: user.Email,
      name: `${user.First} ${user.Last}`,
      orgId: user.OrganisationID,
      // the response returns the names of the teams, not the ids, so we ignore them
      teamIds: [],
    } satisfies core.User
  }

  listPlans = async (): Promise<core.Plan[]> => {
    this.logger.verbose(`listing plans`)
    const { data: plans } = await firstValueFrom(
      this.http.get<tyk.Plan[]>('plans'),
    )
    this.logger.verbose(`list plans response: ${JSON.stringify(plans)}`)
    return plans.map(
      plan =>
        ({
          id: plan.ID,
          name: plan.Name,
          autoApproveAccessRequests: plan.AutoApproveAccessRequests,
        }) satisfies core.Plan,
    )
  }

  getPlanById = async (id: number): Promise<core.PlanDetail> => {
    this.logger.verbose(`getting plan ${id}`)
    const { data: plan } = await firstValueFrom(
      this.http.get<tyk.PlanDetail>(`plans/${id}`),
    )
    this.logger.verbose(`get plan ${id} response: ${JSON.stringify(plan)}`)
    return {
      id: plan.ID,
      name: plan.Name,
      autoApproveAccessRequests: plan.AutoApproveAccessRequests,
      metadata: plan.MetaData,
    } satisfies core.PlanDetail
  }

  listProducts = async (): Promise<core.Product[]> => {
    this.logger.verbose(`listing products`)
    const { data: products } = await firstValueFrom(
      this.http.get<tyk.Product[]>('products'),
    )
    this.logger.verbose(`list products response: ${JSON.stringify(products)}`)
    return products.map(
      product =>
        ({
          id: product.ID,
          name: product.Name,
        }) satisfies core.Product,
    )
  }

  createAccessRequest = async (input: {
    appId: number
    planId: number
    productIds: number[]
  }): Promise<boolean> => {
    this.logger.verbose(
      `creating access request response for app ${input.appId}`,
    )
    try {
      await firstValueFrom(
        this.http.put<tyk.AccessRequestResponse>(
          `/apps/${input.appId}/provision`,
          {
            PlanID: input.planId,
            ProductIDs: input.productIds,
          },
        ),
      )
      return true
    } catch (error) {
      this.logger.error(
        `Failed to create access request for app ${input.appId}: ${error}`,
      )
      return false
    }
  }
}
