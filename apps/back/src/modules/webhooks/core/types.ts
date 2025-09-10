export type User = {
  id: number
  email: string
  name: string
  orgId: number
  teamIds: number[]
}

export type NewOrganisation = {
  id: number
  name: string
  teams: Array<{
    id: number
    name: string
    default: boolean
  }>
}

export type Application = {
  id: number
  name: string
  userId: number
}

export type Plan = {
  id: number
  autoApproveAccessRequests: boolean
  name: string
}

export type PlanDetail = Plan & {
  metadata: Record<string, string>
}

export type Product = {
  id: number
  name: string
}
