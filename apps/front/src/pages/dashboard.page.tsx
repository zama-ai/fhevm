import { useLoaderData } from 'react-router'
import { MeQuery } from '@/__generated__/graphql'

export function DashboardPage() {
  const { me } = useLoaderData<MeQuery>()

  return (
    <div>
      <div>(dashboard page)</div>
      <b>hello {me.name}</b>
    </div>
  )
}
