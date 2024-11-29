import { createBrowserRouter, RouterProvider } from 'react-router'
import { PublicLayout } from './layouts/public.layout'

import { DefaultPage } from './pages/default.page'
import { ErrorPage } from './pages/error.page'

import { SignupPage } from './pages/signup.page'
import { SigninPage } from './pages/signin.page'
const router = createBrowserRouter([
  {
    path: '/',
    element: <DefaultPage />,
    errorElement: <ErrorPage />,
  },
  {
    element: <PublicLayout />,
    children: [
      {
        path: 'signup/:invitationKey',
        element: <SignupPage />,
        errorElement: <ErrorPage />,
      },
      {
        path: 'signin',
        element: <SigninPage />,
        errorElement: <ErrorPage />,
      },
    ],
  },
])

export function Router() {
  return <RouterProvider router={router} />
}
