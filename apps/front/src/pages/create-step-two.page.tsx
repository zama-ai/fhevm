import { useState } from 'react'
import { useNavigate } from 'react-router'
import { Heading, Box } from '@chakra-ui/react'

import { CreatorAddress } from '@/components/creator/creator-address'
import { CreatorStepper } from '@/components/creator-stepper/creator-stepper'

export function CreateStepTwoPage() {
  const [isLoading, setIsLoading] = useState(false)
  const navigate = useNavigate()
  return (
    <>
      <Heading mb="5">Create a new dApp</Heading>
      <Box display="flex" justifyContent="start" mb="5">
        <CreatorStepper currentStep={1} />
      </Box>

      <CreatorAddress
        onSubmit={() => {
          setIsLoading(true)
          navigate('/create/2')
        }}
        loading={isLoading}
        errorMessage="Error shall display here"
      />
    </>
  )
}
