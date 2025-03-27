import { useId } from 'react'
import { Box, BoxProps } from '@chakra-ui/react'
import { AreaChart, Area, ResponsiveContainer, YAxis } from 'recharts'
import type { CurveProps } from 'recharts'

import { createCategoryColors } from './utils.js'

export interface SparklineProps extends BoxProps {
  /**
   * The chart data.
   */
  data: Array<Record<string, string | number>>
  /**
   * The categories to display, values map to keys in data.
   * @default ['value']
   */
  categories?: string[]
  /**
   * Colors to use for each category.
   * @default ['primary', 'gray']
   */
  colors?: string[]
  /**
   * The curve type of the area.
   */
  curveType?: CurveProps['type']
  /**
   * The width of the line.
   */
  strokeWidth?: number
  /**
   * The sparkline variant.
   * @default gradient
   */
  variant?: 'line' | 'solid' | 'gradient'
  /**
   * Whether to connect null values.
   * @default true
   */
  connectNulls?: boolean
  /**
   * Whether to stack the categories.
   * @default false
   */
  stack?: boolean
}

/**
 * Sparkline component using Recharts
 */
export const Sparkline = (props: SparklineProps) => {
  const {
    data = [],
    categories = ['value'],
    curveType,
    colors = ['primary', 'gray'],
    strokeWidth = 1,
    variant = 'gradient',
    connectNulls = true,
    stack = false,
    ...rest
  } = props

  const id = useId()

  const categoryColors = createCategoryColors(categories, colors)
  const getColor = (category: string) => {
    return categoryColors[category]
  }

  const getGradientId = (category: string) => {
    return `${id}-${category}-gradient`
  }

  const getFill = (category: string) => {
    switch (variant) {
      case 'solid':
        return getColor(category)
      case 'gradient':
        return `url(#${getGradientId(category)})`
      default:
        return 'transparent'
    }
  }

  return (
    <Box {...rest}>
      <ResponsiveContainer width="100%" height="100%" minWidth="0">
        <AreaChart
          data={data}
          margin={{ top: 2, right: 0, left: 0, bottom: 0 }}
        >
          <defs>
            {categories.map(category => (
              <linearGradient
                key={category}
                id={getGradientId(category)}
                x1="0"
                y1="0"
                x2="0"
                y2="1"
              >
                <stop
                  offset="5%"
                  stopColor={getColor(category)}
                  stopOpacity="0.8"
                />
                <stop
                  offset="95%"
                  stopColor={getColor(category)}
                  stopOpacity="0"
                />
              </linearGradient>
            ))}
          </defs>

          <YAxis
            tick={false}
            axisLine={false}
            width={0}
            domain={['dataMin', 'dataMax']}
          />

          {categories.map(category => (
            <Area
              key={category}
              dataKey={category}
              stroke={getColor(category)}
              strokeWidth={strokeWidth}
              fill={getFill(category)}
              fillOpacity="0.2"
              type={curveType}
              connectNulls={connectNulls}
              stackId={stack ? 'a' : undefined}
            />
          ))}
        </AreaChart>
      </ResponsiveContainer>
    </Box>
  )
}
