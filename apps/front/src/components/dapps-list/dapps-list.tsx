import { Stack, Box, Button, Heading } from '@chakra-ui/react'

import { NewIcon } from '../icons/icons.js'
import { DappsListItem } from './dapps-list-item.js'
import { Dapp } from '@/__generated__/graphql.js'

type OwnProps = {
  createDapp: () => void
  dapps: Dapp[]
}
export function DappsList({ createDapp, dapps }: OwnProps) {
  return (
    <Box>
      <Heading as="h2" size="lg" mb="5">
        My dApps
      </Heading>
      <Stack gap="5">
        {dapps.map(dapp => (
          <DappsListItem key={dapp.id} dapp={dapp} />
        ))}
      </Stack>
      <Box my="5">
        <Button onClick={createDapp}>
          <Box as="span">
            <NewIcon strokeWidth={3} />
          </Box>
          Create a new dAapp
        </Button>
      </Box>
    </Box>
  )
}
