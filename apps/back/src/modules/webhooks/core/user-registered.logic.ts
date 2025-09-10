import type { UserRegistered } from './webhooks.types.js'

export function newOrganisationName(data: UserRegistered): string {
  let name = `${data.first} ${data.last}`.trim()

  return `${name || 'User'}'s organisation`
}
