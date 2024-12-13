import { useLoaderData, useNavigate } from 'react-router'
import { Text } from '@chakra-ui/react'
import { MeTeamDappsQuery } from '@/__generated__/graphql'
import { HeroGreetings } from '@/components/hero-greetings/hero-greetings'
import { DappsList } from '@/components/dapps-list/dapps-list'

export function DashboardPage() {
  const { me } = useLoaderData<MeTeamDappsQuery>()
  const navigate = useNavigate()

  return (
    <>
      <HeroGreetings name={me.name} />
      <Text>
        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Facere magnam
        iste hic qui, sapiente impedit! Recusandae corrupti perspiciatis
        nesciunt, quisquam dolorum nam, quidem debitis ut omnis libero quas
        suscipit asperiores.
      </Text>
      <DappsList createDapp={() => navigate('/create')} />
    </>
  )
}
