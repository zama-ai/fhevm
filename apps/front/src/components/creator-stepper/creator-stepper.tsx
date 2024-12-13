import { StepsItem, StepsList, StepsRoot } from '@/components/ui/steps'
import { Card } from '@chakra-ui/react'

const steps = [
  { title: 'Step 1', description: 'Smart contract' },
  { title: 'Step 2', description: 'Link dApp' },
  { title: 'Step 3', description: 'Confirmation' },
]

type OwnProps = { currentStep: number }

export function CreatorStepper({ currentStep }: OwnProps) {
  return (
    <Card.Root size="sm" w={{ sm: 'full', md: '2/3', xl: '1/2' }}>
      <StepsRoot
        defaultValue={0}
        count={steps.length}
        colorPalette="brand"
        step={currentStep}
      >
        <Card.Body color="fg.muted">
          <StepsList fontSize="xs">
            {steps.map(({ title, description }, index) => (
              <StepsItem
                key={index}
                index={index}
                title={title}
                description={description}
              />
            ))}
          </StepsList>
        </Card.Body>
      </StepsRoot>
    </Card.Root>
  )
}
