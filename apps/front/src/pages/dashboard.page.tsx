import { useLoaderData } from 'react-router'
import { MeQuery } from '@/__generated__/graphql'

export function DashboardPage() {
  const { me } = useLoaderData() as MeQuery

  return (
    <div>
      <div>(dashboard page)</div>
      <b>hello {me.email}</b>
    </div>
  )
}
