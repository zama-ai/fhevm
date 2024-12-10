import { CreatorName } from '@/components/creator-name/creator-name'
import { CreatorStepper } from '@/components/creator-stepper/creator-stepper'
import { Heading } from '@chakra-ui/react'

export function CreateStepOnePage() {
  return (
    <>
      <Heading>Create a new dApp</Heading>
      <CreatorStepper currentStep={0} />

      <CreatorName
        onSubmit={console.log}
        loading={false}
        errorMessage="Error"
      />
    </>
  )
}
