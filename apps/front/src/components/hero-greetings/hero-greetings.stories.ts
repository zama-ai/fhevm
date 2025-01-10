import type { Meta, StoryObj } from '@storybook/react'

import { HeroGreetings } from './hero-greetings'

const meta = {
  title: 'Atoms/HeroGreetings',
  component: HeroGreetings,
  parameters: {
    layout: 'centered',
  },
  tags: ['autodocs'],
} satisfies Meta<typeof HeroGreetings>

export default meta
type Story = StoryObj<typeof meta>

export const Default: Story = {
  args: {
    name: 'John Paul',
  },
}
