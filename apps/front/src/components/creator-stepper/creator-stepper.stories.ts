import type { Meta, StoryObj } from '@storybook/react'

import { CreatorStepper } from './creator-stepper'

const meta = {
  title: 'Molecules/CreatorStepper',
  component: CreatorStepper,
  parameters: {
    layout: 'padded',
  },
  tags: ['autodocs'],
  argTypes: {
    currentStep: {
      options: [0, 1, 2],
      control: 'select',
    },
  },
} satisfies Meta<typeof CreatorStepper>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    currentStep: 1,
  },
}
