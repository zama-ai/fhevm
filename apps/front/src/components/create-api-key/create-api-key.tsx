import { ApiKeyForm } from './api-key-form'
import { useCreateApiKey } from './use-create-api-key'

type OwnProps = {
  dappId: string
}

export function CreateApiKey({ dappId }: OwnProps) {
  const { createApiKey, error } = useCreateApiKey(dappId)
  if (error) {
    console.log(`CreateApiKey: error=${JSON.stringify(error)}`)
  }
  return <ApiKeyForm error={error?.message} onCreate={createApiKey} />
}
