import { useId } from 'react'
import { Box, BoxProps } from '@chakra-ui/react'
import {
  PieChart,
  Pie,
  ResponsiveContainer,
  Cell,
  Label,
  Tooltip,
} from 'recharts'

export interface PieChartProps extends BoxProps {
  /**
   * The chart data in format [{name: string, value: number}]
   */
  data: Array<{ name: string; value: number }>
  /**
   * Colors to use for each segment
   * @default ['#0088FE', '#00C49F', '#FFBB28', '#FF8042']
   */
  colors?: string[]
  /**
   * Inner radius of pie
   * @default 0
   */
  innerRadius?: number
  /**
   * Outer radius of pie
   * @default 80
   */
  outerRadius?: number
  /**
   * Whether to show the label
   * @default false
   */
  label?: boolean
  /**
   * Height of the container
   * @default 300
   */
  containerHeight?: number
}

/**
 * Pie chart component using Recharts
 */
export const PieChartComponent = ({
  data,
  colors = [
    '#0088FE',
    '#00C49F',
    '#FFBB28',
    '#FF8042',
    '#FF6384',
    '#36A2EB',
    '#FFCE56',
    '#4BC0C0',
    '#9966FF',
    '#FF9F40',
    '#E57373',
    '#81C784',
    '#64B5F6',
    '#FFD54F',
    '#BA68C8',
    '#4DD0E1',
    '#7986CB',
    '#F06292',
    '#A1887F',
    '#90A4AE',
    '#DCE775',
    '#AED581',
    '#FF8A65',
    '#D4E157',
    '#FF7043',
  ],
  innerRadius = 0,
  outerRadius = 80,
  label = false,
  containerHeight = 300,
  ...rest
}: PieChartProps) => {
  const id = useId()
  return (
    <Box {...rest}>
      <ResponsiveContainer width="100%" height={containerHeight}>
        <PieChart>
          <Pie
            data={data}
            cx="50%"
            cy="50%"
            labelLine={false}
            innerRadius={innerRadius}
            outerRadius={outerRadius}
            dataKey="value"
            label={label}
            paddingAngle={4}
          >
            {data.map((_entry, index) => (
              <Cell
                key={`cell-${id}-${index}`}
                fill={colors[index % colors.length]}
              >
                <Label>any string or number</Label>
              </Cell>
            ))}
          </Pie>
          <Tooltip />
        </PieChart>
      </ResponsiveContainer>
    </Box>
  )
}
