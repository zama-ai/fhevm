import { Outlet, createRootRoute } from "@tanstack/react-router"
import { TanStackRouterDevtools } from "@tanstack/react-router-devtools"

import Header from "../components/header"

const RootLayout = () => (
    <>
        <Header />
        <main className="root-main">
            <Outlet />
            <TanStackRouterDevtools />
        </main>
    </>
)

export const Route = createRootRoute({ component: RootLayout })
