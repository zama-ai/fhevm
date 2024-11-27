import { ApolloClient, InMemoryCache, HttpLink } from '@apollo/client'

export const apolloClient = new ApolloClient({
  link: new HttpLink({
    uri: import.meta.env.VITE_API_URL,
  }),
  cache: new InMemoryCache(),
})
