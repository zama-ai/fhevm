import { useNavigate } from 'react-router'
import { Heading, Box } from '@chakra-ui/react'

import { CreatorStepper } from '#components/creator-stepper/creator-stepper.js'
import { CreatorThankyou } from '#components/creator/creator-thankyou.js'
import { MeTeamDappsQuery } from '#__generated__/graphql.js'
import { useQuery } from '@apollo/client'
import { graphql } from '#__generated__/gql.js'

const GET_ME_TEAMS_DAPPS = graphql(`
  query MeTeamDapps {
    me {
      id
      email
      name
      teams {
        id
        name
        dapps {
          id
          name
          status
        }
      }
    }
  }
`)

export function CreateStepThreePage() {
  const { data } = useQuery<MeTeamDappsQuery>(GET_ME_TEAMS_DAPPS)
  const navigate = useNavigate()
  return (
    <>
      <Heading mb="5">Created a new dApp in {data?.me.teams[0].name}</Heading>
      <Box display="flex" justifyContent="start" mb="5">
        <CreatorStepper currentStep={2} />
      </Box>

      <CreatorThankyou
        onSubmit={() => {
          navigate('/dashboard')
        }}
      />
    </>
  )
}
