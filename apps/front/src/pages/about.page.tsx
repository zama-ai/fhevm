import { useState } from 'react'
import { useQuery } from '@apollo/client'

import { graphql } from '~generated/gql'
import { PublicLayout } from '../layouts/public.layout'

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

export function AboutPage() {
  const [count, setCount] = useState(0)
  const { loading, error, data } = useQuery(GET_FILMS)
  return (
    <PublicLayout>
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
    </PublicLayout>
  )
}
