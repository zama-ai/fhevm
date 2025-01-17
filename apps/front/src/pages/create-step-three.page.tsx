import { useNavigate, useParams } from 'react-router'
import { Heading, Box } from '@chakra-ui/react'

import { CreatorStepper } from '@/components/creator-stepper/creator-stepper.js'
import { CreatorThankyou } from '@/components/creator/creator-thankyou.js'

export function CreateStepThreePage() {
  const { dappId } = useParams()
  const navigate = useNavigate()
  return (
    <>
      <Heading mb="5">Created a new dApp</Heading>
      <Box display="flex" justifyContent="start" mb="5">
        <CreatorStepper currentStep={2} />
      </Box>

      <CreatorThankyou
        onSubmit={() => {
          navigate(`/dapp/${dappId}`)
        }}
      />
    </>
  )
}
