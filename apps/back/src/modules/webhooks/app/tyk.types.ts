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
  Teams: Team[]
}
