import { z } from 'zod'
import { AppError, validationError } from '@/utils/app-error'
import { Entity } from '@/utils/entity'
import { ok, fail, Result } from '@/utils/result'

const schema = z.object({
  id: z.string().uuid(),
  name: z.string(),
  // status: z.enum(['DRAFT', 'PUBLISHED']),
  teamId: z.string().uuid(),
  address: z.string().optional().nullable(),
})

export type DappProps = z.infer<typeof schema>

export class Dapp extends Entity<DappProps> implements Readonly<DappProps> {
  static parse(data: unknown): Result<Dapp, AppError> {
    const check = schema.safeParse(data)
    return check.success
      ? ok(new Dapp(check.data))
      : fail(validationError(check.error.message))
  }
  static parseArray(data: unknown[]): Result<Dapp[], AppError> {
    const res = data.map(Dapp.parse)
    return res.every(dapp => dapp.isOk())
      ? ok(res.reduce<Dapp[]>((acc, dapp) => [...acc, dapp.value], []))
      : fail(res.find(dapp => dapp.isFail())!.error)
  }

  get id() {
    return this.get('id')
  }

  get name() {
    return this.get('name')
  }

  // get status() {
  //   return this.get('status')
  // }

  get teamId() {
    return this.get('teamId')
  }

  get address() {
    return this.get('address')
  }
}
