export type Flag = 'invitations'

export function useFeatureFlag(flag: Flag): boolean {
  window.env = window.env || {}

  console.info(
    'flag',
    flag,
    'is',
    window.env[`VITE_FLAG_${flag.toUpperCase()}`],
  )
  return !!window.env[`VITE_FLAG_${flag.toUpperCase()}`]
}
