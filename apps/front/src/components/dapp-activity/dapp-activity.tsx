import { Stack } from '@chakra-ui/react'

import { BlockSparkline } from '@/components/stats-blocks/block-sparkline'

import {
  calculateOperationStats,
  calculateEncryptionStats,
  calculateTotal,
  byDayToSparkline,
} from '@/lib/stats.js'
import { CumulativeDappStats } from '@/__generated__/graphql.js'
import { BlockUsageChart } from '../stats-blocks/block-usage-chart'

type DailyStats = {
  id: string
  day: string
  cumulative: {
    total: number
  }
}

type OwnProps = {
  cumulativeDappStats: CumulativeDappStats
  byDayDappStats: DailyStats[]
}

export function DappActivity({
  cumulativeDappStats,
  byDayDappStats,
}: OwnProps) {
  const {
    // eslint-disable-next-line @typescript-eslint/no-unused-vars
    __typename: _trash01,
    total,
    ...cumulative
  } = cumulativeDappStats || {
    total: 0,
  }

  const byDay = byDayDappStats.map(({ id, day, cumulative: { total } }) => ({
    id,
    day,
    total,
  }))

  const operationStatsData = calculateOperationStats(cumulative)
  const operationStatsTotal = calculateTotal(operationStatsData)
  const encryptionStatsData = calculateEncryptionStats(cumulative)
  const encryptionStatsTotal = calculateTotal(encryptionStatsData)

  return (
    <Stack direction="column" gap="5">
      <Stack direction="row" gap="5">
        <BlockUsageChart
          total={total}
          operationStatsData={operationStatsData}
          operationStatsTotal={operationStatsTotal}
          encryptionStatsData={encryptionStatsData}
          encryptionStatsTotal={encryptionStatsTotal}
        />
        <BlockSparkline data={byDayToSparkline(byDay)} />
      </Stack>
    </Stack>
  )
}
