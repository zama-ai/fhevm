import type { Meta, StoryObj } from '@storybook/react'
import { PieChartComponent } from './pie'

const meta: Meta<typeof PieChartComponent> = {
  title: 'Atoms/Visualization/PieChart',
  component: PieChartComponent,
  parameters: {
    layout: 'centered',
  },
}

export default meta
type Story = StoryObj<typeof PieChartComponent>

const defaultData = [
  { name: 'Group A', value: 400 },
  { name: 'Group B', value: 300 },
  { name: 'Group C', value: 300 },
  { name: 'Group D', value: 200 },
]

export const Default: Story = {
  args: {
    data: defaultData,
    width: '400px',
    height: '400px',
  },
}

export const DonutChart: Story = {
  args: {
    data: defaultData,
    innerRadius: 60,
    outerRadius: 80,
    width: '400px',
    height: '400px',
  },
}

export const CustomColors: Story = {
  args: {
    data: defaultData,
    colors: ['#003f5c', '#58508d', '#bc5090', '#ff6361'],
    width: '400px',
    height: '400px',
  },
}

export const CustomRadius: Story = {
  args: {
    data: defaultData,
    innerRadius: 30,
    outerRadius: 100,
    width: '400px',
    height: '400px',
  },
}
