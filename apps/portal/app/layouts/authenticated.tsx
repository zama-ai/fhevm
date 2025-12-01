import { Outlet, type LoaderFunctionArgs } from "react-router";
import type { Route } from "./+types/authenticated";
import { getUserFromSession } from "~/features/auth/session.server";
import { redirectWithFlash } from "~/features/flash-messages/flash.server";

export async function loader({ request }: LoaderFunctionArgs) {
  const user = await getUserFromSession(request);

  if (!user) {
    // Redirect to login page if not authenticated
    return redirectWithFlash(request, "/auth/login", {
      type: "warning",
      message: "Please sign in to continue.",
    });
  }

  return { user };
}

export default function AuthenticatedLayout({
  loaderData,
}: Route.ComponentProps) {
  // The main layout already handles the navbar, so we just pass through the outlet
  return <Outlet context={{ user: loaderData.user }} />;
}
