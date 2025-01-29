import { ValueObject } from 'utils'
import { z } from 'zod'

export class SubscriptionId extends ValueObject('SubscriptionId', z.number()) {}
