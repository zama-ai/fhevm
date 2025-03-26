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
