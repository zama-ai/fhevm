export function CheckEnv() {
  return [import.meta.env.VITE_API_URL]
    .filter((k) => !k)
    .map((_, i) => <div key={i}>missing env key</div>)
}
