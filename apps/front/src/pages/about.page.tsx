import { useState } from 'react'
import { useQuery } from '@apollo/client'
import { NavLink } from 'react-router'

import { graphql } from '../__generated__/gql.js'
import { AboutMeQuery } from '#__generated__/graphql.js'

const GET_ME = graphql(`
  query AboutMe {
    me {
      id
      email
      name
      teams {
        id
        name
      }
    }
  }
`)

export function AboutPage() {
  const [count, setCount] = useState(0)
  const { loading, error, data } = useQuery<AboutMeQuery>(GET_ME)
  return (
    <div>
      <button onClick={() => setCount(count => count + 1)}>
        count is {count}
      </button>
      {loading ? (
        <p>Loading...</p>
      ) : error ? (
        <p>Error :(</p>
      ) : (
        <p>
          {' '}
          my email address:
          {data?.me.email}
        </p>
      )}
      <hr />
      go to{' '}
      <NavLink to="/signup/e7e720ef-e8d6-4e07-883d-ed93ea7a6999">
        signup
      </NavLink>{' '}
      page
    </div>
  )
}
