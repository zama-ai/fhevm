import { Text, Heading } from '@chakra-ui/react'
import { Link } from '../ui/link'

export function SignupConfirmed() {
  return (
    <>
      <Heading>Check your Email</Heading>
      <Text>We've sent you a confirmation link.</Text>
      <Text>
        Please check your inbox and click the link to continue your
        registration.
      </Text>

      <Text>
        Didn’t get the email? Check your spam folder or{' '}
        <Link to="#">contact support</Link>.
      </Text>
    </>
  )
}
