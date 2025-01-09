import { createBrowserRouter, RouterProvider } from 'react-router'
import { PublicLayout } from './layouts/public.layout.js'
import { PrivateLayout } from './layouts/private.layout.js'

// utility pages
import { DefaultPage } from './pages/default.page.js'
import { ErrorPage } from './pages/error.page.js'
import { privateLoader } from './pages/private.loader.js'
import { UnauthorizedErrorPage } from './pages/unauthorized.error.page.js'

// publlic pages
import { SigninPage } from './pages/signin.page.js'
import { SignupPage } from './pages/signup.page.js'
import { SignupErrorPage } from './pages/signup.error.page.js'
import { signupLoader } from './pages/signup.loader.js'

import { AboutPage } from './pages/about.page.js'
import { DashboardPage } from './pages/dashboard.page.js'
import { CreateStepOnePage } from './pages/create-step-one.page.js'
import { CreateStepTwoPage } from './pages/create-step-two.page.js'
import { CreateStepThreePage } from './pages/create-step-three.page.js'
import { DappPage } from './pages/dapp.page.js'

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
    loader: privateLoader,
    errorElement: <UnauthorizedErrorPage />,
    children: [
      {
        path: '/dashboard/',
        element: <DashboardPage />,
      },
      {
        path: '/about',
        element: <AboutPage />,
      },
      {
        path: '/create/:teamId',
        element: <CreateStepOnePage />,
      },
      {
        path: '/create/2/:dappId',
        element: <CreateStepTwoPage />,
      },
      {
        path: '/create/3/:dappId',
        element: <CreateStepThreePage />,
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
