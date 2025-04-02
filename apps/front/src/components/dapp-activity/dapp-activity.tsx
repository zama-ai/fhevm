import { Stack } from '@chakra-ui/react'

import { BlockSimple } from '@/components/stats-blocks/block-simple'
import { BlockPie } from '@/components/stats-blocks/block-pie'
import { BlockSparkline } from '@/components/stats-blocks/block-sparkline'

import {
  calculateOperationStats,
  calculateEncryptionStats,
  calculateTotal,
  byDayToSparkline,
} from '@/lib/stats.js'
import { CumulativeDappStats, DailyDappStats } from '@/__generated__/graphql.js'

type OwnProps = {
  cumulativeDappStats: CumulativeDappStats
  byDayDappStats: DailyDappStats[]
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
        <BlockSimple title="Total FHE Events" amount={total} />
        <BlockPie
          title="FHE Operations"
          total={operationStatsTotal || 0}
          data={operationStatsData}
        />
        <BlockPie
          title="FHE Encryption"
          total={encryptionStatsTotal || 0}
          data={encryptionStatsData}
        />
      </Stack>
      <BlockSparkline data={byDayToSparkline(byDay)} />
    </Stack>
  )
}
