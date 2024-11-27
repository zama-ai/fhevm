import { useRouteError } from 'react-router'

export function ErrorPage() {
  const error = useRouteError() as { message: string; statusText: string }
  console.error(error)

  return (
    <div id="error-page">
      <h1>Oops!</h1>
      <p>An unexpected error has occurred.</p>
      <p>
        <i>{error.statusText || error.message}</i>
      </p>
    </div>
  )
}
