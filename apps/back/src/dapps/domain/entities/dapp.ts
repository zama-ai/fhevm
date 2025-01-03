import { z } from 'zod'
import type { AppError, Result } from 'utils'
import { Entity, ok, fail, validationError } from 'utils'
import { CreatedAt, DAppId } from './value-objects'
import { da } from '@faker-js/faker/.'

const status = z.enum(['DRAFT', 'DEPLOYING', 'LIVE'])

const schema = z.object({
  id: DAppId,
  name: z.string(),
  status,
  teamId: z.string().uuid(),
  address: z
    .string()
    .length(42, 'sepolia address must be exactly 42 charaxters long')
    .startsWith('0x', 'sepolia address must start with 0x')
    .optional()
    .nullable(),
  createdAt: CreatedAt,
})

export type DAppProps = z.infer<typeof schema>
export type DAppStatus = z.infer<typeof status>

export class DApp
  extends Entity<DAppProps>
  implements
    Readonly<
      Omit<DAppProps, 'id' | 'createdAt'> & { id: DAppId; createdAt: CreatedAt }
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
    address,
  }: {
    teamId: string
    name: string
    address?: string
  }): Result<DApp, AppError> {
    return DApp.parse({
      id: DAppId.generate().value,
      name,
      status: 'DRAFT',
      teamId,
      address,
      createdAt: CreatedAt.generate().value,
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
    return this.get('teamId')
  }

  get address() {
    return this.get('address')
  }

  get createdAt() {
    return new CreatedAt(this.get('createdAt'))
  }
}
