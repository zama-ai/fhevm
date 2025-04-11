type EnvVariable = 'VITE_BACK_HTTP_URL' | 'VITE_BACK_WS_URL'
declare global {
  interface Window {
    env: Record<EnvVariable, string>
  }
}

export function CheckEnv() {
  return ['VITE_BACK_HTTP_URL', 'VITE_BACK_WS_URL']
    .filter(
      (envKey): envKey is EnvVariable =>
        window.env[envKey as EnvVariable] === undefined,
    )
    .map((key, i) => (
      <div key={i}>
        Missing env key <code>{key}</code>
      </div>
    ))
}
