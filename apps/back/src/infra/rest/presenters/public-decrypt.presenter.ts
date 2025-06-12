export function PublicDecryptPresenter(
  data: { signatures: string[]; decryptedValue: string }[],
): { signatures: string[]; decrypted_value: string }[] {
  if (!Array.isArray(data)) {
    console.warn(
      `PublicDecryptPresenter: data is not an array - ${JSON.stringify(data)}`,
    )
    throw new Error('PublicDecryptPresenter: data is not an array')
  }
  return data.map(({ signatures, decryptedValue }) => ({
    signatures,
    decrypted_value: decryptedValue,
  }))
}
