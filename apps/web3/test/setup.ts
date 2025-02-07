import { allCustomMatcher } from 'aws-sdk-client-mock-vitest'
import { expect } from 'vitest'

expect.extend(allCustomMatcher)
