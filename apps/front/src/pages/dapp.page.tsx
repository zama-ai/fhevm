import { gql, useQuery, useSubscription } from '@apollo/client'
import { useParams } from 'react-router'
import { Box } from '@chakra-ui/react'
import { graphql } from '@/__generated__/gql.js'

import {
  DappUpdatedSubscription,
  GetDappDetailsQuery,
} from '@/__generated__/graphql.js'

import { DappHeader } from '@/components/dapp-header/dapp-header'
import { CreateApiKey } from '@/components/create-api-key/create-api-key'
import { ListApiKeys } from '@/components/list-api-keys/list-api-keys'
import { DappActivity } from '@/components/dapp-activity/dapp-activity'

const GET_DAPP_DETAILS = graphql(`
  query GetDappDetails($dappId: ID!) {
    dapp(input: { id: $dappId }) {
      id
      name
      status
      rawStats {
        id
        name
        timestamp
        externalRef
      }
      stats {
        id
        cumulative {
          total
          FheAdd
          FheSub
          FheMul
          FheDiv
          FheRem
          FheBitAnd
          FheBitOr
          FheBitXor
          FheShl
          FheShr
          FheRotl
          FheRotr
          FheEq
          FheEqBytes
          FheNe
          FheNeBytes
          FheGe
          FheGt
          FheLe
          FheLt
          FheMin
          FheMax
          FheNeg
          FheNot
          VerifyCiphertext
          Cast
          TrivialEncrypt
          TrivialEncryptBytes
          FheIfThenElse
          FheRand
          FheRandBounded
        }
        byDay {
          id
          day
          total
          computation
          encryption
        }
      }
    }
  }
`)

const SUB_DAPP_UPDATED = gql(`
  subscription DappUpdated($dappId: ID!) {
    dappUpdated(input: { id: $dappId }) {
      id
      name
      status
      rawStats {
        id
        name
        timestamp
        externalRef
      }
      stats {
        id
        cumulative {
          total
          FheAdd
          FheSub
          FheMul
          FheDiv
          FheRem
          FheBitAnd
          FheBitOr
          FheBitXor
          FheShl
          FheShr
          FheRotl
          FheRotr
          FheEq
          FheEqBytes
          FheNe
          FheNeBytes
          FheGe
          FheGt
          FheLe
          FheLt
          FheMin
          FheMax
          FheNeg
          FheNot
          VerifyCiphertext
          Cast
          TrivialEncrypt
          TrivialEncryptBytes
          FheIfThenElse
          FheRand
          FheRandBounded
        }
        byDay {
          id
          day
          total
          computation
          encryption
        }
      }
    } 
  }
`)

export function DappPage() {
  const { dappId } = useParams()
  const { data, error } = useQuery<GetDappDetailsQuery>(GET_DAPP_DETAILS, {
    variables: { dappId },
  })
  const { data: liveData } = useSubscription<DappUpdatedSubscription>(
    SUB_DAPP_UPDATED,
    {
      variables: { dappId },
    },
  )

  if (error) {
    throw Error(error.message)
  }

  return (
    <Box>
      <DappHeader dapp={data?.dapp} dappUpdated={liveData?.dappUpdated} />
      {data && (
        <DappActivity
          cumulativeDappStats={data.dapp.stats.cumulative}
          byDayDappStats={data.dapp.stats.byDay}
        />
      )}
      {dappId && (
        <>
          <CreateApiKey key="create-api-key" dappId={dappId} />
          <ListApiKeys key="list-api-keys" dappId={dappId} />
        </>
      )}
    </Box>
  )
}
