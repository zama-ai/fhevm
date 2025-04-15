import { faker } from '@faker-js/faker'
import { GraphQl } from './graphql.js'
import { SetupManager } from './setup.manager.js'

export class HttpzManager {
  constructor(private readonly manager: SetupManager) {}

  get httpServer() {
    return this.manager.httpServer
  }

  createApiKey({
    token,
    dappId,
    name = faker.string.alphanumeric(10),
    description,
  }: {
    token: string
    dappId: string
    name?: string
    description?: string
  }) {
    return GraphQl.request<
      { createApiKey: CreateApiKeyResult },
      { dappId: string; name: string; description?: string }
    >(this.httpServer)
      .mutate(CREATE_API_KEY, { dappId, name, description })
      .auth(token)
      .exec('createApiKey')
  }

  getApiKey({ token, id }: { token: string; id: string }) {
    return GraphQl.request<{ apiKey: GetApiKeyResult }, { id: string }>(
      this.httpServer,
    )
      .query(GET_API_KEY, { id })
      .auth(token)
      .exec('apiKey')
  }
}

export type CreateApiKeyResult = {
  token: string
  apiKey: {
    id: string
    dappId: string
    name: string
    description?: string
  }
}

const CREATE_API_KEY = `
  mutation createApiKey($dappId: String!, $name: String!, $description: String) {
    createApiKey(input: {
      dappId: $dappId
      name: $name
      description: $description
    }){
      token
      apiKey {
        id
        dappId
        name
        description
      }
    }
  }
`

export type GetApiKeyResult = {
  id: string
  dappId: string
  name: string
  description?: string
}
const GET_API_KEY = `
  query getApiKey($id: ID!) {
  apiKey(input: {id: $id}) {
    id
    dappId
    name
    description
  }
}
`
