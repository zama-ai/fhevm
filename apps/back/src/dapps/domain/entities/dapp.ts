import { z } from 'zod'
import type { AppError, Result, Unbrand } from 'utils'
import { Entity, ok, fail, validationError, some, none } from 'utils'
import { DAppId } from './value-objects.js'
import { TeamId } from '#teams/domain/entities/value-objects.js'
import { ChainId } from '#chains/domain/entities/value-objects.js'
import { Web3Address } from '#shared/entities/value-objects/web3-address.js'

const schema = z.object({
  id: DAppId.schema,
  name: z.string(),
  teamId: TeamId.schema,
  chainId: ChainId.schema,
  address: Web3Address.schema,
  createdAt: z
    .date()
    .refine(date => date <= new Date(), 'CreatedAt should be in the past'),
})

export type DAppProps = Unbrand<z.infer<typeof schema>>

export class DApp
  extends Entity<DAppProps>
  implements
    Readonly<
      Omit<DAppProps, 'id' | 'teamId' | 'chainId' | 'address'> & {
        id: DAppId
        teamId: TeamId
        chainId: ChainId
        address: Web3Address
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
    chainId: number
    address: string
  }): Result<DApp, AppError> {
    return DApp.parse({
      id: DAppId.random().value,
      name,
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

  get teamId() {
    return new TeamId(this.get('teamId'))
  }

  get chainId(): ChainId {
    const chainId = this.get('chainId')
    return new ChainId(chainId)
  }

  get address(): Web3Address {
    const address = this.get('address')
    return new Web3Address(address)
  }

  get createdAt() {
    return this.get('createdAt')
  }
}
