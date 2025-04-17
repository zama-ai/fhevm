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
      <ApiKeyForm
        error={errorMessage}
        onCreate={details =>
          createApiKey({ dappId, description: '', ...details })
        }
        // resetting state with a key when new token is created
        // https://react.dev/learn/preserving-and-resetting-state
        key={token}
      />
    </>
  )
}
