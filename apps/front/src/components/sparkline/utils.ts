/**
 * Create a mapping of categories to colors
 * @param categories
 * @param colors
 * @returns Array of category, color pairs
 */
export const createCategoryColors = (
  categories: string[],
  colors: string[],
) => {
  return Object.fromEntries(
    categories.map((category, index) => {
      const color = colors[index] ?? 'gray.500'
      return [category, color]
    }),
  )
}
