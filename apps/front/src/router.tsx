import { createBrowserRouter, RouterProvider } from 'react-router'
import { PublicLayout } from './layouts/public.layout'
import { PrivateLayout } from './layouts/private.layout'

// utility pages
import { DefaultPage } from './pages/default.page'
import { ErrorPage } from './pages/error.page'
import { privateLoader } from './pages/private.loader'
import { UnauthorizedErrorPage } from './pages/unauthorized.error.page.tsx'

// publlic pages
import { SigninPage } from './pages/signin.page'
import { SignupPage } from './pages/signup.page'
import { SignupErrorPage } from './pages/signup.error.page'
import { signupLoader } from './pages/signup.loader'

import { AboutPage } from './pages/about.page'
import { DashboardPage } from './pages/dashboard.page'
import { dashboardLoader } from './pages/dashboard.loader'
import { CreateStepOnePage } from './pages/create-step-one.page.tsx'

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
        loader: dashboardLoader,
      },
      {
        path: '/about',
        element: <AboutPage />,
      },
      {
        path: '/app/create/1',
        element: <CreateStepOnePage />,
      },
    ],
  },
])

export function Router() {
  return <RouterProvider router={router} />
}
