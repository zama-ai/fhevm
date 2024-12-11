import { useNavigate } from 'react-router'
import { Heading, Box } from '@chakra-ui/react'

import { CreatorStepper } from '@/components/creator-stepper/creator-stepper'
import { CreatorThankyou } from '@/components/creator/creator-thankyou'

export function CreateStepThreePage() {
  const navigate = useNavigate()
  return (
    <>
      <Heading mb="5">Create a new dApp</Heading>
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
