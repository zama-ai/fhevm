export type Team = {
  ID: number
  Name: string
}

export type User = {
  ID: number
  Active: boolean
  Email: string
  First: string
  Last: string
  OrganisationID: number
  Teams: Array<string | number>
  Role: string
}

export type NewOrganizationResponse = {
  ID: number
  Name: string
  Teams: Array<{
    ID: number
    Name: string
    Default: boolean
  }>
}

export type Plan = {
  ID: number
  Name: string
  AutoApproveAccessRequests: boolean
}

export type PlanDetail = Plan & {
  DisplayName: string
  Description: string
  MetaData: Record<string, string>
}

export type Product = {
  ID: number
  Name: string
  DisplayName: string
  Feature: boolean
}

export type AccessRequestResponse = {
  status: 'ok'
  message: 'string'
}
