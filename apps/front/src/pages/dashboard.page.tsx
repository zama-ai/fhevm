import { useLoaderData } from 'react-router'
import { Text } from '@chakra-ui/react'
import { MeQuery } from '@/__generated__/graphql'
import { HeroGreetings } from '@/components/hero-greetings/hero-greetings'

export function DashboardPage() {
  const { me } = useLoaderData<MeQuery>()

  return (
    <>
      <HeroGreetings name={me.name} />
      <Text>
        Lorem ipsum, dolor sit amet consectetur adipisicing elit. Facere magnam
        iste hic qui, sapiente impedit! Recusandae corrupti perspiciatis
        nesciunt, quisquam dolorum nam, quidem debitis ut omnis libero quas
        suscipit asperiores.
      </Text>
    </>
  )
}
