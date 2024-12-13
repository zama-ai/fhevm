import { createContext } from 'react'
export const TitleContext = createContext({
  title: 'My first dApp',
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  setTitle: (_: string) => {},
})
