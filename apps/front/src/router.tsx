import { createBrowserRouter, RouterProvider } from 'react-router'
import { PublicLayout } from './layouts/public.layout.js'
import { PrivateLayout } from './layouts/private.layout.js'

// utility pages
import { DefaultPage } from './pages/default.page.js'
import { ErrorPage } from './pages/error.page.js'
import { PrivateErrorPage } from './pages/private.error.page.js'

// TODO: use lazy loading and code splitting
// public pages
import { SigninPage } from './pages/signin.page.js'
import { SignupPage } from './pages/signup.page.tsx'
import { InvitationPage } from './pages/invitation.page.js'
import { InvitationErrorPage } from './pages/invitation.error.page.js'
import { invitationLoader } from './pages/invitation.loader.js'

import { DashboardPage } from './pages/dashboard.page.js'
import { CreatePage } from './pages/create.js'
import { DappPage } from './pages/dapp.page.js'
import { CheckEmailPage } from './pages/check-email.page.tsx'
import { ValidationPage } from './pages/validation.page.tsx'
import { ValidationErrorPage } from './pages/validation.error.page.tsx'
import { validationLoader } from './pages/validation.loader.ts'

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
      { path: 'signup', element: <SignupPage /> },
      { path: 'check-email', element: <CheckEmailPage /> },
      {
        path: 'validate/:validationToken',
        element: <ValidationPage />,
        errorElement: <ValidationErrorPage />,
        loader: validationLoader,
      },
      {
        path: 'invitation/:invitationToken',
        element: <InvitationPage />,
        errorElement: <InvitationErrorPage />,
        loader: invitationLoader,
      },
      {
        path: 'signin',
        element: <SigninPage />,
      },
      {
        path: 'reset-password/:token?',
        lazy: async () => {
          const Component = (
            await import('./features/auth/reset-password/page.tsx')
          ).ResetPasswordPage
          return { Component }
        },
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
        lazy: async () => {
          const Component = (await import('./features/preferences/page.tsx'))
            .PreferencesPage
          return { Component }
        },
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
