import { useRouteError } from 'react-router'
import { Code, Heading, List, Stack, Text } from '@chakra-ui/react'
import { Link } from '@/components/ui/link.js'

export function PrivateErrorPage() {
  const error = useRouteError() as { message: string; statusText: string }
  console.error(error)

  return (
    <Stack id="error-page" m="5">
      <Heading>{error.message}</Heading>
      <ErrorMessage error={error} />
      <Code p="5" my="5">
        error code: {error.message}
        {error.statusText ? ` - ${error.statusText}` : ''}
      </Code>
    </Stack>
  )
}

function UnauthorizedError() {
  return (
    <>
      <Text>You are not authorized to access this page.</Text>
      <Text>Here is a list of possible causes 🤔</Text>
      <List.Root>
        <List.Item>
          Your session has expired{' '}
          <Link variant="underline" to="/signin">
            try signin in
          </Link>
        </List.Item>
        <List.Item>You are trying to access a private page</List.Item>
      </List.Root>
      <Text>Anyway! Contact us for help, we'll gladly help you!</Text>
    </>
  )
}
function NotFoundError() {
  return (
    <>
      <Text>
        We are sorry, but the page you are looking for does not exist.
      </Text>
      <Text>
        Go back to the{' '}
        <Link variant="underline" to="/dashboard">
          dashboard
        </Link>
      </Text>
    </>
  )
}

function FailedToFetchError() {
  return (
    <>
      <Text>
        We are sorry, but we were unable to fetch the data you requested.
      </Text>
      {navigator.onLine ? (
        <Text>Please hit refresh or try again later.</Text>
      ) : (
        <Text>You are offline, please check your internet connection.</Text>
      )}
    </>
  )
}
function InternalServerError() {
  return <Text>We are sorry, but something went wrong on our side.</Text>
}
function ForbiddenError() {
  return <Text>You are not authorized to access this page.</Text>
}
function BadRequestError() {
  return <Text>We are sorry, but the request was invalid.</Text>
}
function DefaultError() {
  return <Text>We are sorry, but something went wrong.</Text>
}

function ErrorMessage({ error }: { error: { message: string } }) {
  switch (error.message) {
    case 'Unauthorized':
      return <UnauthorizedError />
    case 'Not Found':
    case 'DApp not found':
    case 'Team not found':
      return <NotFoundError />
    case 'Failed to fetch':
      return <FailedToFetchError />
    case 'Internal Server Error':
      return <InternalServerError />
    case 'Forbidden':
      return <ForbiddenError />
    case 'Bad Request':
      return <BadRequestError />
    default:
      return <DefaultError />
  }
}
