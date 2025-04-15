import { Box, Skeleton, Stack } from '@chakra-ui/react'
import {
  DappUpdatedSubscription,
  GetDappDetailsQuery,
} from '@/__generated__/graphql'

import { InplaceInput } from '@/components/inplace-input/inplace-input'
import { DappStatus } from '@/components/dapp-status/dapp-status.js'
import { useDappUpdate } from '@/hooks/use-dapp-update'

type OwnProps = {
  dapp?: GetDappDetailsQuery['dapp']
  dappUpdated?: DappUpdatedSubscription['dappUpdated']
}

export function DappHeader({ dapp, dappUpdated }: OwnProps) {
  // TODO: handle error
  const { updateDapp, error } = useDappUpdate(dapp?.id || '')
  if (!dapp) {
    return <Skeleton height="5" my="5" width="30rem" />
  }
  const data = dappUpdated || dapp

  return (
    <Stack direction="row" align="center" alignItems="flex-start">
      <Box mb="5">
        <InplaceInput
          title={data.name}
          onChange={() => {}}
          onUpdate={({ value }) => {
            updateDapp({ name: value })
          }}
        />
      </Box>
      <DappStatus status={data.status} ml="2" size="xs" />
    </Stack>
  )
}
