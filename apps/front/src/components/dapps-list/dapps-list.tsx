import { Box, Button } from '@chakra-ui/react'

import { NewIcon } from '../icons/icons'

type OwnProps = {
  createDapp: () => void
}
export function DappsList({ createDapp }: OwnProps) {
  return (
    <Box py="5">
      <Button onClick={createDapp}>
        <Box as="span">
          <NewIcon strokeWidth={3} />
        </Box>
        Create a new dAapp
      </Button>
    </Box>
  )
}
