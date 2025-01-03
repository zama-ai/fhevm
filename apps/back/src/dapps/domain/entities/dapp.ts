import { z } from 'zod'
import type { AppError, Result } from 'utils'
import { Entity, ok, fail, validationError } from 'utils'

const status = z.enum(['DRAFT', 'DEPLOYING', 'LIVE'])

const schema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  status,
  teamId: z.string().uuid(),
  address: z
    .string()
    .length(42, 'sepolia address must be exactly 42 charaxters long')
    .startsWith('0x', 'sepolia address must start with 0x')
    .optional()
    .nullable(),
})

export type DAppProps = z.infer<typeof schema>
export type DAppStatus = z.infer<typeof status>

export class DApp extends Entity<DAppProps> implements Readonly<DAppProps> {
  static parse(data: unknown): Result<DApp, AppError> {
    if (!data) return fail(validationError('data is undefined'))
    const check = schema.safeParse(data)
    return check.success
      ? ok(new DApp(check.data))
      : fail(validationError(check.error.message))
  }

  static parseArray(data: unknown[]): Result<DApp[], AppError> {
    const res = data.map(DApp.parse)
    return res.every(dapp => dapp.isOk())
      ? ok(res.reduce<DApp[]>((acc, dapp) => [...acc, dapp.value], []))
      : fail(res.find(dapp => dapp.isFail())!.error)
  }

  get id() {
    return this.get('id')
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
}
