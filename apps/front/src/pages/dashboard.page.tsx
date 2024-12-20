import { useLoaderData, useNavigate } from 'react-router'
import { Text } from '@chakra-ui/react'
import { MeTeamDappsQuery, Team } from '@/__generated__/graphql'
import { HeroGreetings } from '@/components/hero-greetings/hero-greetings'
import { DappsList } from '@/components/dapps-list/dapps-list'
import { getPersonalTeam } from '@/lib/personal-team'

export function DashboardPage() {
  const { me } = useLoaderData<MeTeamDappsQuery>()
  const navigate = useNavigate()
  const team = me ? getPersonalTeam(me.teams as Team[]) : null
  return (
    <>
      <HeroGreetings name={me.name} />
      <Text my="5">
        We do not support FHE yet, but you can create a new Dapp and start using
        it.
      </Text>
      {team && (
        <DappsList createDapp={() => navigate('/create')} dapps={team.dapps} />
      )}
    </>
  )
}
