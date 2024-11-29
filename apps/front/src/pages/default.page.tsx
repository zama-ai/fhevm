import { useState } from 'react'
import { useQuery } from '@apollo/client'
import { NavLink } from 'react-router'

import { graphql } from '~generated/gql'

const GET_FILMS = graphql(`
  query FilmsQuery {
    allFilms {
      films {
        id
        title
        releaseDate
      }
    }
  }
`)

export function DefaultPage() {
  const [count, setCount] = useState(0)
  const { loading, error, data } = useQuery(GET_FILMS)
  return (
    <div>
      <button onClick={() => setCount((count) => count + 1)}>
        count is {count}
      </button>
      {loading ? (
        <p>Loading...</p>
      ) : error ? (
        <p>Error :(</p>
      ) : (
        <ul>
          {data?.allFilms?.films?.map((film, index) => (
            <li key={index}>{film?.title}</li>
          ))}
        </ul>
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
