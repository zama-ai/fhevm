import { Outlet } from "react-router";
import type { Route } from "./+types/main";
import {
  getUserAndFlash,
  type FlashMessage,
} from "~/features/flash-messages/flash.server";
import type { Auth0User } from "~/types/auth";
import { Navbar } from "~/components/navbar";
import FlashMessageBanner from "~/features/flash-messages/components/flash-message-banner";

type MainLoaderData = {
  user?: Auth0User;
  flash: FlashMessage | null;
};

export async function loader({ request }: Route.LoaderArgs) {
  const { user, flash, headers } = await getUserAndFlash(request);

  if (headers) {
    return Response.json({ user, flash: flash ?? null }, { headers });
  }

  return Response.json({ user, flash: flash ?? null });
}

export default function MainLayout({ loaderData }: Route.ComponentProps) {
  const { user, flash } = loaderData as MainLoaderData;
  return (
    <div className="min-h-screen flex flex-col bg-zama-500">
      <Navbar user={user} />
      <FlashMessageBanner flash={flash} />
      <main className="flex-1">
        <Outlet context={{ user }} />
      </main>
    </div>
  );
}
