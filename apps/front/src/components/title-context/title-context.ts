import { createContext } from 'react'
export const TitleContext = createContext({
  title: 'context default',
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  setTitle: (_: string) => {},
})
