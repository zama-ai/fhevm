import { shortString } from 'utils'

export function PublicDecryptPresenter(
  data: { signatures: string[]; decryptedValue: string }[],
): { signatures: string[]; decrypted_value: string }[] {
  if (!Array.isArray(data)) {
    console.warn(
      `PublicDecryptPresenter: data is not an array - ${JSON.stringify(data, (_, v) => (typeof v === 'string' ? shortString(v) : v))}`,
    )
    throw new Error('PublicDecryptPresenter: data is not an array')
  }
  return data.map(({ signatures, decryptedValue }) => ({
    signatures,
    decrypted_value: decryptedValue,
  }))
}
