import { useCallback, useEffect, useState } from 'react'
import { ApiKeyForm } from './api-key-form'
import { useCreateApiKey } from '@/components/create-api-key/use-create-api-key'
import { ApiKeyCreatedDialog } from './api-key-created-dialog'

type OwnProps = {
  dappId: string
}

export function CreateApiKey({ dappId }: OwnProps) {
  const { createApiKey, errorMessage, token } = useCreateApiKey()
  const [showModal, setShowModal] = useState(false)
  const closeModal = useCallback(() => {
    setShowModal(false)
  }, [setShowModal])

  useEffect(() => {
    if (token) {
      setShowModal(true)
    }
  }, [token])
  return (
    <>
      {showModal && <ApiKeyCreatedDialog token={token!} onClose={closeModal} />}
      {/** https://react.dev/learn/preserving-and-resetting-state#option-2-resetting-state-with-a-key */}
      <ApiKeyForm
        error={errorMessage}
        onCreate={details =>
          createApiKey({ variables: { dappId, ...details } })
        }
        key={token}
      />
    </>
  )
}
