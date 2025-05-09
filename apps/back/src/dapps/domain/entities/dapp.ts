import { z } from 'zod'
import type { AppError, Option, Result, Unbrand } from 'utils'
import { Entity, ok, fail, validationError, some, none } from 'utils'
import { DAppId } from './value-objects.js'
import { TeamId } from '#users/domain/entities/value-objects.js'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import { Web3Address } from '#shared/entities/value-objects/web3-address.js'

const status = z.enum([
  'DRAFT',
  'DEPLOYING',
  'LIVE',
  'FAILED',
  'ARCHIVED',
  'DELETED',
])

const schema = z.object({
  id: DAppId.schema,
  name: z.string(),
  status,
  teamId: TeamId.schema,
  chainId: ChainId.schema.nullish(),
  address: Web3Address.schema.nullish(),
  createdAt: z
    .date()
    .refine(date => date <= new Date(), 'CreatedAt should be in the past'),
})

export type DAppProps = Unbrand<z.infer<typeof schema>>
export type DAppStatus = z.infer<typeof status>

export class DApp
  extends Entity<DAppProps>
  implements
    Readonly<
      Omit<DAppProps, 'id' | 'teamId' | 'chainId' | 'address'> & {
        id: DAppId
        teamId: TeamId
        chainId: Option<ChainId>
        address: Option<Web3Address>
      }
    >
{
  static parse(data: unknown): Result<DApp, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new DApp(check.data))
      : fail(validationError(check.error.message))
  }

  static create({
    teamId,
    name,
    chainId,
    address,
  }: {
    teamId: string
    name: string
    chainId?: number
    address?: string
  }): Result<DApp, AppError> {
    return DApp.parse({
      id: DAppId.random().value,
      name,
      status: 'DRAFT',
      teamId,
      chainId,
      address,
      createdAt: new Date(),
    })
  }

  get id() {
    return new DAppId(this.get('id'))
  }

  get name() {
    return this.get('name')
  }

  get status() {
    return this.get('status')
  }

  get teamId() {
    return new TeamId(this.get('teamId'))
  }

  get chainId(): Option<ChainId> {
    const chainId = this.get('chainId')
    return chainId ? some(new ChainId(chainId)) : none()
  }

  get address(): Option<Web3Address> {
    const address = this.get('address')
    return address ? some(new Web3Address(address)) : none()
  }

  get createdAt() {
    return this.get('createdAt')
  }
}
