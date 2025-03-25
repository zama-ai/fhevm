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
}

/**
 * Pie chart component using Recharts
 */
export const PieChartComponent = ({
  data,
  colors = ['#0088FE', '#00C49F', '#FFBB28', '#FF8042'],
  innerRadius = 0,
  outerRadius = 80,
  label = false,
  ...rest
}: PieChartProps) => {
  const id = useId()

  return (
    <Box {...rest} position="absolute" border="1px solid red">
      <ResponsiveContainer width="100%" height={60}>
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
