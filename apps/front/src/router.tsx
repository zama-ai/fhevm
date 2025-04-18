import { createBrowserRouter, RouterProvider } from 'react-router'
import { PublicLayout } from './layouts/public.layout.js'
import { PrivateLayout } from './layouts/private.layout.js'

// utility pages
import { DefaultPage } from './pages/default.page.js'
import { ErrorPage } from './pages/error.page.js'
import { PrivateErrorPage } from './pages/private.error.page.js'

// public pages
import { SigninPage } from './pages/signin.page.js'
import { SignupPage } from './pages/signup.page.js'
import { SignupErrorPage } from './pages/signup.error.page.js'
import { signupLoader } from './pages/signup.loader.js'

import { DashboardPage } from './pages/dashboard.page.js'
import { CreatePage } from './pages/create.js'
import { DappPage } from './pages/dapp.page.js'
import { PreferencesPage } from './pages/preferences.page.js'

const router = createBrowserRouter([
  {
    index: true,
    element: <DefaultPage />,
    errorElement: <ErrorPage />,
  },
  {
    element: <PublicLayout />,
    errorElement: <ErrorPage />,
    children: [
      {
        path: 'signup/:invitationToken',
        element: <SignupPage />,
        errorElement: <SignupErrorPage />,
        loader: signupLoader,
      },
      {
        path: 'signin',
        element: <SigninPage />,
      },
    ],
  },
  {
    element: <PrivateLayout />,
    errorElement: <PrivateErrorPage />,
    children: [
      {
        path: '/dashboard/',
        element: <DashboardPage />,
      },
      {
        path: '/preferences/',
        element: <PreferencesPage />,
      },
      {
        path: '/create/:teamId',
        element: <CreatePage />,
      },
      {
        path: '/dapp/:dappId',
        element: <DappPage />,
      },
    ],
  },
])

export function Router() {
  return <RouterProvider router={router} />
}
