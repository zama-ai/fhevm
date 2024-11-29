// TODO: tweak when backend returns proper GraphqlError messages
export function formatErrorMessage(message: string) {
  return message.replace('GraphQL error: ', '')
}
