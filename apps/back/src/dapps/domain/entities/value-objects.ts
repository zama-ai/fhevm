import { ValueObject } from 'utils'
import { z } from 'zod'

export class DAppId extends ValueObject('DAppId', z.string().uuid()) {}
