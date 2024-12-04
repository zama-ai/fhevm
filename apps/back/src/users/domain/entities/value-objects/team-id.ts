import { ValueObject } from '@/utils/value-object'
import { z } from 'zod'

export class TeamId extends ValueObject('TeamId', z.string().uuid()) {}
