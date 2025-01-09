import { useState, ReactNode, createElement } from 'react'
import { TitleContext } from './title-context.js'

type OwnProps = {
  children: ReactNode
}

export function TitleContextWrapper({ children }: OwnProps) {
  const [title, setTitle] = useState('state default')
  const value = { title, setTitle }
  return createElement(TitleContext.Provider, { value: value }, children)
}
