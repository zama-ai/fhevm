// TODO: tweak when backend returns proper GraphqlError messages
// https://github.com/zama-zws/console/issues/20
export function formatErrorMessage(message: string) {
  return message.replace('GraphQL error: ', '')
}
