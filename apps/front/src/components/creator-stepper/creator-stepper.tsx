import { StepsItem, StepsList, StepsRoot } from '@/components/ui/steps'
import { Card } from '@chakra-ui/react'

type OwnProps = { currentStep: number }

export function CreatorStepper({ currentStep }: OwnProps) {
  return (
    <Card.Root size="sm">
      <StepsRoot
        defaultValue={1}
        count={3}
        colorPalette="orange"
        step={currentStep}
      >
        <Card.Body color="fg.muted">
          <StepsList fontSize="xs">
            <StepsItem index={0} title="Step 1" description="Smart contract" />
            <StepsItem index={1} title="Step 2" description="Link dApp" />
            <StepsItem index={2} title="Step 3" description="Confirmation" />
          </StepsList>
        </Card.Body>
      </StepsRoot>
    </Card.Root>
  )
}
