import { type LoaderFunctionArgs } from "react-router";
import { destroyUserSession } from "~/features/auth/session.server";
import { getLogoutUrl } from "~/features/auth/auth0.server";

export async function loader({ request }: LoaderFunctionArgs) {
  // Destroy the session and get the cookie header
  const destroySessionCookie = await destroyUserSession(request);

  // Redirect to Auth0 logout with the session destruction cookie
  const logoutUrl = getLogoutUrl(request);

  return new Response(null, {
    status: 302,
    headers: {
      "Set-Cookie": destroySessionCookie,
      Location: logoutUrl,
    },
  });
}

export default function Logout() {
  return null;
}
