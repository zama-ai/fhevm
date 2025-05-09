import { DAppStatus } from '#dapps/domain/entities/dapp.js'
import { faker } from '@faker-js/faker'
import { GraphQlResponse, SetupManager } from './setup.manager.js'
import { AuthManager } from './auth.manager.js'
import { GraphQl } from './graphql.js'

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

export interface DAppStats {
  id: string
  rawStats: {
    id: string
    name: string
    timestamp: Date
    externalRef: string
  }[]
}

export interface CumulativeDappStats {
  total: number
  FheAdd: number
  FheSub: number
  FheMul: number
  FheDiv: number
  FheRem: number
  FheBitAnd: number
  FheBitOr: number
  FheBitXor: number
  FheShl: number
  FheShr: number
  FheRotl: number
  FheRotr: number
  FheEq: number
  FheEqBytes: number
  FheNe: number
  FheNeBytes: number
  FheGe: number
  FheGt: number
  FheLe: number
  FheLt: number
  FheMin: number
  FheMax: number
  FheNeg: number
  FheNot: number
  VerifyCiphertext: number
  Cast: number
  TrivialEncrypt: number
  TrivialEncryptBytes: number
  FheIfThenElse: number
  FheRand: number
  FheRandBounded: number
}

export interface DappStats {
  id: string
  cumulative: CumulativeDappStats
}

export type ValidateAddress =
  | { check: true; message?: never }
  | { check: false; message: string }

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
    chainId,
    address,
  }: ({ token: string; teamId: string } | { token?: never; teamId?: never }) & {
    name: string
    chainId?: number
    address?: string
  }): Promise<GraphQlResponse<DApp>> {
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

    return GraphQl.request<
      { createDapp: DApp },
      {
        teamId: string
        name: string
        chainId?: number
        address: string | undefined
      }
    >(this.httpServer)
      .auth(token)
      .mutate(CREATE_DAPP, { teamId: teamId!, name, chainId, address })
      .exec('createDapp')
  }

  async getDapp({
    token,
    dappId,
  }: {
    token: string
    dappId: string
  }): Promise<GraphQlResponse<DApp>> {
    return GraphQl.request<{ dapp: DApp }, { appId: string }>(this.httpServer)
      .auth(token)
      .query(GET_DAPP, { appId: dappId })
      .exec('dapp')
  }

  async updateDApp({
    token,
    dappId,
    name,
    chainId,
    address,
  }: {
    token: string
    dappId: string
    name?: string
    chainId?: number
    address?: string
  }) {
    return GraphQl.request<
      { updateDapp: DApp },
      { appId: string; name?: string; chainId?: number; address?: string }
    >(this.httpServer)
      .auth(token)
      .mutate(UPDATE_DAPP, { appId: dappId, name, chainId, address })
      .exec('updateDapp')
  }

  async deployDApp({ token, dappId }: { token: string; dappId: string }) {
    return GraphQl.request<
      { deployDapp: DeployDappResult },
      { dappId: string }
    >(this.httpServer)
      .auth(token)
      .mutate(DEPLOY_DAPP, { dappId })
      .exec('deployDapp')
  }

  async getDappRawStats({
    token,
    dappId,
  }: {
    token: string
    dappId: string
  }): Promise<GraphQlResponse<DAppStats>> {
    return GraphQl.request<{ dapp: DAppStats }, { dappId: string }>(
      this.httpServer,
    )
      .auth(token)
      .query(GET_DAPP_RAW_STATS, { dappId })
      .exec('dapp')
  }

  async valiateAddress({
    token,
    chainId,
    address,
  }: {
    token: string
    chainId: number
    address: string
  }): Promise<GraphQlResponse<ValidateAddress>> {
    return GraphQl.request<
      { validateAddress: ValidateAddress },
      { chainId: number; address: string }
    >(this.httpServer)
      .auth(token)
      .query(VALIDATE_ADDRESS, { chainId, address })
      .exec('validateAddress')
  }

  async getDappStats({
    token,
    dappId,
  }: {
    token: string
    dappId: string
  }): Promise<GraphQlResponse<{ stats: DappStats }>> {
    return GraphQl.request<{ dapp: { stats: DappStats } }, { dappId: string }>(
      this.httpServer,
    )
      .auth(token)
      .query(GET_DAPP_STATS, { dappId })
      .exec('dapp')
  }
}

const CREATE_DAPP = `
  mutation createDApp($teamId: String!, $name: String!, $chainId: Int, $address: String) {
    createDapp(input: { teamId: $teamId, name: $name, chainId: $chainId, address: $address }) {
      id
      name
      chainId
      address
      status
      team {
        id
        name
      }
    }
  }
`

const GET_DAPP = `
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

const UPDATE_DAPP = `
  mutation updateApp($appId: ID!, $name: String, $chainId: Int, $address: String) {
    updateDapp(input: { id: $appId, name: $name, chainId: $chainId, address: $address }) {
      id
      name
      chainId
      address
      status
      team {
        id
        name
      }
    }
  }
`

const DEPLOY_DAPP = `
  mutation DeployDapp($dappId: String!) {
    deployDapp(input: { dappId: $dappId }) {
      id
      name
      status
    }
  }
`

const GET_DAPP_RAW_STATS = `
  query GetDappStats($dappId: ID!) {
    dapp(input: { id: $dappId }) {
      id
      rawStats {
        id
        name
        timestamp
        externalRef
      }
    }
  }
`

const VALIDATE_ADDRESS = `
  query ValidateAddress($chainId: Int!, $address: String!) {
    validateAddress(input: {chainId: $chainId, address: $address}) {
      check
      message
    }
  }
`

const GET_DAPP_STATS = `
  query GetDappStats($dappId: ID!) {
    dapp(input: { id: $dappId }) {
      stats {
        id
        cumulative {
          total
          FheAdd
          FheSub
          FheMul
          FheDiv
          FheRem
          FheBitAnd
          FheBitOr
          FheBitXor
          FheShl
          FheShr
          FheRotl
          FheRotr
          FheEq
          FheEqBytes
          FheNe
          FheNeBytes
          FheGe
          FheGt
          FheLe
          FheLt
          FheMin
          FheMax
          FheNeg
          FheNot
          VerifyCiphertext
          Cast
          TrivialEncrypt
          TrivialEncryptBytes
          FheIfThenElse
          FheRand
          FheRandBounded
        }
      }
    }
  }
`
