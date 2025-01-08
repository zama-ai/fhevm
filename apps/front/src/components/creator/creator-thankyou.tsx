import { Box, Grid, Heading, Text, Stack } from '@chakra-ui/react'

import { SpinnerButton } from '@/components/ui/spinner-button.js'
import { WaitIcon } from '../icons/icons.js'

type OwnProps = {
  onSubmit: () => void
}

export function CreatorThankyou({ onSubmit }: OwnProps) {
  return (
    <Stack gap="5">
      <Grid templateColumns="repeat(2, 1fr)" gap="6">
        <img
          src="https://media1.tenor.com/m/9m_NhmtIqFsAAAAd/fmyoonie-taegi-dance.gif"
          alt="Dancing"
        />

        <Box>
          <Heading as="h3" size="lg" mb="2">
            Thank You!
          </Heading>
          <Text my="2">
            Your dApp is being linked to Zama, you'll get an email when it's
            ready.
          </Text>
          <Text color="gray.500" fontSize="sm">
            <WaitIcon mr="1" />
            Estimated time: 10 minutes
          </Text>
        </Box>
      </Grid>

      <Box display="flex" justifyContent="flex-end">
        <SpinnerButton
          loadingText="Saving..."
          type="submit"
          alignSelf="flex-start"
          onClick={onSubmit}
        >
          View dApp
        </SpinnerButton>
      </Box>
    </Stack>
  )
}
