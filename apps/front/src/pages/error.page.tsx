import { useRouteError } from 'react-router'
import { Alert, Box, Heading } from '@chakra-ui/react'
import { Link } from '@/components/ui/link'
import './error.page.css'

export function ErrorPage() {
  const error = useRouteError() as { message: string; statusText: string }
  console.error(error)

  return (
    <Box id="error-page" p="2">
      <Box maxW="2xl" mx="auto" mt="10">
        {/** custom css animation */}
        <h1 className="glitch-text" id="oops" data-text="Error!">
          Error!
        </h1>

        <Heading as="h2" size="lg" my="4">
          Sorry, an unexpected error has occurred.
        </Heading>

        <Alert.Root status="error">
          <Alert.Indicator />
          <Alert.Title>
            {error.statusText || error.message || 'unknow error'}
          </Alert.Title>
        </Alert.Root>

        <Link my="4" to="/">
          → Back to home{' '}
        </Link>
      </Box>
    </Box>
  )
}
