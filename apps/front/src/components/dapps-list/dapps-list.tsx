import { Stack, Box, Button, Heading } from '@chakra-ui/react'

import { NewIcon } from '../icons/icons'
import { DappsListItem } from './dapps-list-item'

type OwnProps = {
  createDapp: () => void
}
export function DappsList({ createDapp }: OwnProps) {
  return (
    <Box>
      <Heading as="h2" size="lg" mb="5">
        My dApps
      </Heading>
      <Stack gap="5">
        {[
          {
            id: '1',
            name: 'My first dApp',
            status: 'LIVE',
            updatedAt: '2021-09-01',
            createdAt: '2021-09-01',
          },
          {
            id: '2',
            name: 'My new dApp',
            status: 'DRAFT',
            updatedAt: '2021-09-01',
            createdAt: '2021-09-01',
          },
          {
            id: '3',
            name: 'Another thing being built',
            status: 'DEPLOYING',
            updatedAt: '2021-09-01',
            createdAt: '2021-09-01',
          },
        ].map(dapp => (
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
