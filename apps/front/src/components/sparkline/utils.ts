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
  const categoryColors: { [key: string]: string } = {}
  categories.forEach((category, index) => {
    const color = colors[index] ?? 'gray.500'
    categoryColors[category] = color
  })
  return categoryColors
}
