import { DAppStatus } from '@/dapps/domain/entities/dapp'
import { faker } from '@faker-js/faker'
import gql from 'graphql-tag'
import request from 'supertest-graphql'
import { GraphQlResponse, SetupManager } from './setup.manager'
import { AuthManager } from './auth.manager'

export interface DApp {
  id: string
  name: string
  address: string | null
  status: DAppStatus
  team: {
    id: string
    name: string
  }
}

export interface DeployDappResult {
  id: string
  status: DAppStatus
  name: string
}

export class DappManager {
  constructor(
    private readonly manager: SetupManager,
    private readonly auth: AuthManager,
  ) {}

  get httpServer() {
    return this.manager.httpServer
  }

  async createDApp({
    token,
    teamId,
    name,
    address,
  }: ({ token: string; teamId: string } | { token?: never; teamId?: never }) & {
    name: string
    address?: string
  }): Promise<GraphQlResponse<{ dapp: DApp; token: string }>> {
    if (!token) {
      const result = await this.auth.signup(
        {
          name: faker.internet.username(),
          password: faker.internet.password(),
        },
        { createInvitation: true },
      )
      if (result.success) {
        token = result.data?.token
        teamId = result.data?.user.teams[0].id
      } else {
        return result
      }
    }

    const resp = await request<{ createDapp: DApp }>(this.httpServer)
      .auth(token, { type: 'bearer' })
      .mutate(CREATE_DAPP)
      .variables({ teamId, name, address })

    return resp.data
      ? { success: true, data: { dapp: resp.data.createDapp, token } }
      : { success: false, errors: resp.errors! }
  }

  async getDapp({
    token,
    dappId,
  }: {
    token: string
    dappId: string
  }): Promise<GraphQlResponse<DApp>> {
    const resp = await request<{ dapp: DApp }>(this.httpServer)
      .auth(token, { type: 'bearer' })
      .query(GET_DAPP)
      .variables({ appId: dappId })

    return resp.data
      ? { success: true, data: resp.data.dapp }
      : { success: false, errors: resp.errors! }
  }

  async updateDApp({
    token,
    dappId,
    name,
    address,
  }: {
    token: string
    dappId: string
    name?: string
    address?: string
  }): Promise<GraphQlResponse<{ dapp: DApp }>> {
    const resp = await request<{ updateDapp: DApp }>(this.httpServer)
      .auth(token, { type: 'bearer' })
      .mutate(UPDATE_DAPP)
      .variables({ appId: dappId, name, address })

    return resp.data
      ? { success: true, data: { dapp: resp.data.updateDapp } }
      : { success: false, errors: resp.errors! }
  }

  async deployDApp({
    token,
    dappId,
  }: {
    token: string
    dappId: string
  }): Promise<GraphQlResponse<{ dapp: DeployDappResult }>> {
    const result = await request<
      { deployDapp: DeployDappResult },
      { dappId: string }
    >(this.httpServer)
      .auth(token, { type: 'bearer' })
      .mutate(DEPLOY_DAPP)
      .variables({ dappId })

    return result.data
      ? { success: true, data: { dapp: result.data.deployDapp } }
      : { success: false, errors: result.errors! }
  }
}

const CREATE_DAPP = gql`
  mutation createDApp($teamId: String!, $name: String!, $address: String) {
    createDapp(input: { teamId: $teamId, name: $name, address: $address }) {
      id
      name
      address
      status
      team {
        id
        name
      }
    }
  }
`

const GET_DAPP = gql`
  query getApp($appId: ID!) {
    dapp(input: { id: $appId }) {
      id
      name
      address
      status
      team {
        id
        name
      }
    }
  }
`

const UPDATE_DAPP = gql`
  mutation updateApp($appId: ID!, $name: String, $address: String) {
    updateDapp(input: { id: $appId, name: $name, address: $address }) {
      id
      name
      address
      status
      team {
        id
        name
      }
    }
  }
`

const DEPLOY_DAPP = gql`
  mutation DeployDapp($dappId: String!) {
    deployDapp(input: { dappId: $dappId }) {
      id
      name
      status
    }
  }
`
