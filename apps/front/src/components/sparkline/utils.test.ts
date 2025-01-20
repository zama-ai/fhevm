import { describe, expect, it } from 'vitest'
import { createCategoryColors } from './utils'
describe('createCategoryColors', () => {
  it('should map categories to corresponding colors', () => {
    const categories = ['category1', 'category2', 'category3']
    const colors = ['red', 'green', 'blue']
    const result = createCategoryColors(categories, colors)
    expect(result).toEqual({
      category1: 'red',
      category2: 'green',
      category3: 'blue',
    })
  })

  it('should use default color if colors array is shorter than categories array', () => {
    const categories = ['category1', 'category2', 'category3']
    const colors = ['red']
    const result = createCategoryColors(categories, colors)
    expect(result).toEqual({
      category1: 'red',
      category2: 'gray.500',
      category3: 'gray.500',
    })
  })

  it('should return an empty object if categories array is empty', () => {
    const categories: string[] = []
    const colors: string[] = []
    const result = createCategoryColors(categories, colors)
    expect(result).toEqual({})
  })
})
