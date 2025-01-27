import {
  ApolloClient,
  InMemoryCache,
  HttpLink,
  from,
  split,
} from '@apollo/client'
import { getMainDefinition } from '@apollo/client/utilities'
import { GraphQLWsLink } from '@apollo/client/link/subscriptions'
import { createClient } from 'graphql-ws'
import { setContext } from '@apollo/client/link/context'
import { onError } from '@apollo/client/link/error'

const httpLink = new HttpLink({
  uri: import.meta.env.VITE_BACK_HTTP_URL,
})

const wsLink = new GraphQLWsLink(
  createClient({
    url: import.meta.env.VITE_BACK_WS_URL,
    connectionParams: () => {
      const token = localStorage.getItem('token')
      console.log('wsLink token', token)
      if (!token) return {}
      return {
        authorization: `Bearer ${token}`,
      }
    },
  }),
)

const transportLink = split(
  ({ query }) => {
    const definition = getMainDefinition(query)
    return (
      definition.kind === 'OperationDefinition' &&
      definition.operation === 'subscription'
    )
  },
  wsLink,
  httpLink,
)

const errorLink = onError(({ graphQLErrors, networkError }) => {
  if (graphQLErrors)
    graphQLErrors.forEach(({ message, locations, path }) => {
      console.error(`💣 gql error:${message}`)
      console.log(`Path: ${path}`)
      if (locations) console.log(`Location: ${JSON.stringify(locations)}`)
    })
  if (networkError) console.error(`💣 network error: ${networkError}`)
})

// https://www.apollographql.com/docs/react/networking/authentication
const authLink = setContext((_, { headers }) => {
  // get the authentication token from local storage if it exists
  const token = localStorage.getItem('token')
  // return the headers to the context so httpLink can read them
  console.log('authLink headers', headers)
  return {
    headers: {
      ...headers,
      authorization: token ? `Bearer ${token}` : '',
    },
  }
})

export const apolloClient = new ApolloClient({
  link: from([authLink, errorLink, transportLink]),
  cache: new InMemoryCache(),
  name: 'ConsoleWebClient',
})
