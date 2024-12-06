import { useLoaderData } from 'react-router'

import { MeQuery } from '@/__generated__/graphql'
import { HeroGreetings } from '@/components/hero-greetings/hero-greetings'

export function DashboardPage() {
  const { me } = useLoaderData<MeQuery>()

  return (
    <>
      <HeroGreetings name={me.name} />
    </>
  )
}
