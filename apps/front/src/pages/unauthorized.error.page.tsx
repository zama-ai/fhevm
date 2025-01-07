import { useRouteError } from 'react-router'
import { Code, Heading, List, Stack, Text } from '@chakra-ui/react'
import { Link } from '#components/ui/link.js'

export function UnauthorizedErrorPage() {
  const error = useRouteError() as { message: string; statusText: string }
  console.error(error)

  return (
    <Stack id="error-page">
      <Heading>Unauthorized</Heading>
      <Text>You are not authorized to access this page.</Text>
      <Text>Here is a list of possible causes 🤔</Text>
      <List.Root>
        <List.Item>
          Your session has expired <Link to="/signin">try signin in</Link>
        </List.Item>
        <List.Item>You are trying to access a private page</List.Item>
      </List.Root>
      <Text>Anyway! Contact us for help, we'll gladly help you!</Text>

      <Code p="5" my="5">
        error code: {error.message}
        {error.statusText ? ` - ${error.statusText}` : ''}
      </Code>
    </Stack>
  )
}
