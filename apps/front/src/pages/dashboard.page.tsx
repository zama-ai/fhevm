import { useNavigate } from 'react-router'
import { useQuery } from '@apollo/client'

import { HeroGreetings } from '@/components/hero-greetings/hero-greetings.js'
import { DappsList } from '@/components/dapps-list/dapps-list.js'
import { getPersonalTeam } from '@/lib/personal-team.js'
import { Dapp, MeQuery } from '@/__generated__/graphql.js'
import { GET_ME } from '@/queries.js'

export function DashboardPage() {
  const navigate = useNavigate()
  const { loading, error, data } = useQuery<MeQuery>(GET_ME)
  if (error) throw new Error(error.message)
  const team = data?.me ? getPersonalTeam(data.me.teams) : null

  return (
    <>
      <HeroGreetings name={data?.me.name} loading={loading} />

      {team && (
        <DappsList
          createDapp={() => navigate(`/create/${team.id}`)}
          dapps={team.dapps as Dapp[]}
        />
      )}
    </>
  )
}
