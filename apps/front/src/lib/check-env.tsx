export function CheckEnv() {
  return ['VITE_BACK_HTTP_URL', 'VITE_BACK_WS_URL']
    .filter(envKey => import.meta.env[envKey] === undefined)
    .map((key, i) => (
      <div key={i}>
        Missing env key <code>{key}</code>
      </div>
    ))
}
