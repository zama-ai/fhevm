/* eslint-disable @typescript-eslint/no-empty-object-type */
import 'vitest'
import { CustomMatcher } from 'aws-sdk-client-mock-vitest'

declare module 'vitest' {
  interface Assertion<T = any> extends CustomMatcher<T> {}
  interface AsymmetricMatchersContaining extends CustomMatcher {}
}
