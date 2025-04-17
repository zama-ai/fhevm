import { operationName, operationNames } from './shared.js'

export function getRandomOperation(): operationName {
  const randomIndex = Math.floor(Math.random() * operationNames.length)
  return operationNames[randomIndex]
}
