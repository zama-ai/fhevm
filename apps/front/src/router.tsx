import { createBrowserRouter, RouterProvider } from 'react-router'
import { PublicLayout } from './layouts/public.layout'

// utility pages
import { DefaultPage } from './pages/default.page'
import { ErrorPage } from './pages/error.page'

// publlic pages
import { SigninPage } from './pages/signin.page'
import { SignupPage } from './pages/signup.page'
import { SignupErrorPage } from './pages/signup.error.page'
import { signupLoader } from './pages/signup.loader'

import { AboutPage } from './pages/about.page'
import { DashboardPage } from './pages/dashboard.page'
import { dashboardLoader } from './pages/dashboard.loader'

const router = createBrowserRouter([
  {
    path: '/',
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
    path: '/dashboard/',
    element: <DashboardPage />,
    loader: dashboardLoader,
  },
  {
    path: '/about',
    element: <AboutPage />,
  },
])

export function Router() {
  return <RouterProvider router={router} />
}
