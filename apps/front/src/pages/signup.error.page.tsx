import { useRouteError } from 'react-router'
import { Heading, List, Stack, Text } from '@chakra-ui/react'
import { Link } from '@/components/ui/link.js'

export function SignupErrorPage() {
  const error = useRouteError() as { message: string; statusText: string }
  console.error(error)

  return (
    <Stack id="error-page">
      <Heading>This invitation is not valid</Heading>
      <Text>Here is a list of possible causes 🤔</Text>
      <List.Root>
        <List.Item>The invitation has expired</List.Item>
        <List.Item>
          The invitation has already been used,{' '}
          <Link to="/signin">try signin in</Link>
        </List.Item>
        <List.Item>There as a typo in the invitation link</List.Item>
      </List.Root>
      <Text>Anyway! Contact us for help, we'll gladly help you!</Text>
    </Stack>
  )
}
