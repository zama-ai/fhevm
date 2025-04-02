type StatData = Array<{ name: string; value: number }>
type CumulativeStats = {
  [key: string]: number
}

export function calculateOperationStats(
  data: CumulativeStats | undefined,
): StatData {
  if (!data) return []

  return Object.entries(data)
    .filter(([key]) => !['TrivialEncrypt', 'VerifyCiphertext'].includes(key))
    .map(([key, value]) => ({
      name: key,
      value: value
        ? typeof value === 'string'
          ? parseInt(value, 10)
          : value
        : 0,
    }))
}

export function calculateEncryptionStats(
  data: CumulativeStats | undefined,
): StatData {
  if (!data) return []

  return Object.entries(data)
    .filter(([key]) => ['TrivialEncrypt', 'VerifyCiphertext'].includes(key))
    .map(([key, value]) => ({
      name: key,
      value: value as number,
    }))
}

export function calculateTotal(stats: StatData): number {
  return stats.reduce((acc, curr) => acc + curr.value, 0)
}

export function toYYMMDD(date: Date): string {
  return date.toISOString().split('T')[0]
}

export function byDayToSparkline(
  stats: Array<{ id: string; day: string; total: number; fhe: number }>,
): Array<{ value: number; compareValue: number }> {
  if (stats.length === 0) return []

  const dates = stats.map(s => new Date(s.day))
  const minDate = new Date(Math.min(...dates.map(d => d.getTime())))
  const maxDate = new Date(Math.max(...dates.map(d => d.getTime())))

  const statsMap = new Map(
    stats.map(s => [
      s.day,
      { index: s.day, value: s.total, compareValue: s.fhe },
    ]),
  )

  // Generate array of all days in range
  const result: Array<{ index: string; value: number; compareValue: number }> =
    []
  const currentDate = new Date(minDate)
  currentDate.setUTCHours(0, 0, 0, 0)

  while (currentDate <= maxDate) {
    const dayStr = toYYMMDD(currentDate)
    const existingStat = statsMap.get(dayStr)
    const { index, value, compareValue } = existingStat || {
      index: dayStr,
      value: 0,
      compareValue: 0,
    }
    result.push({ index, value, compareValue })
    currentDate.setUTCDate(currentDate.getUTCDate() + 1)
  }

  return result.sort((a, b) => a.index.localeCompare(b.index))
}
