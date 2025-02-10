import { allCustomMatcher } from 'aws-sdk-client-mock-vitest'
import { expect } from 'vitest'
import { config } from 'dotenv'

config({ path: './.env.test' })
expect.extend(allCustomMatcher)
