import { Team } from '@/__generated__/graphql'

/**
 * returns the user personal team
 * currently, there is only 1 team per use, so it just returns the first one
 * @param {Team[]} teams
 * @returns {Team} team
 */
export function getPersonalTeam(teams: Team[]): Team {
  return teams[0]
}
