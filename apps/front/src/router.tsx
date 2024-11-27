import { createBrowserRouter, RouterProvider } from 'react-router-dom'
import { AboutPage } from './pages/about.page'
import { ErrorPage } from './pages/error.page'

const router = createBrowserRouter([
  {
    path: '/',
    element: <AboutPage />,
    errorElement: <ErrorPage />,
  },
])

export function Router() {
  return <RouterProvider router={router} />
}
