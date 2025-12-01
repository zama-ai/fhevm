import { createCookieSessionStorage } from "react-router";
import type { Auth0User } from "~/types/auth";

if (!process.env.SESSION_SECRET) {
  throw new Error("SESSION_SECRET must be set in environment variables");
}

export const sessionStorage = createCookieSessionStorage({
  cookie: {
    name: "__session",
    secure: process.env.NODE_ENV === "production",
    secrets: [process.env.SESSION_SECRET],
    sameSite: "lax",
    path: "/",
    httpOnly: true,
  },
});

export function getSession(request: Request) {
  const cookie = request.headers.get("Cookie");
  return sessionStorage.getSession(cookie);
}

export async function getUserFromSession(
  request: Request
): Promise<Auth0User | undefined> {
  const session = await getSession(request);
  const user = session.get("user");
  return user;
}

export async function requireUser(request: Request): Promise<Auth0User> {
  const user = await getUserFromSession(request);
  if (!user) {
    throw new Response("Unauthorized", { status: 401 });
  }
  return user;
}

export async function createUserSession(user: Auth0User, redirectTo: string) {
  const session = await sessionStorage.getSession();
  session.set("user", user);

  return new Response(null, {
    status: 302,
    headers: {
      "Set-Cookie": await sessionStorage.commitSession(session),
      Location: redirectTo,
    },
  });
}

export async function destroyUserSession(request: Request) {
  const session = await getSession(request);
  return sessionStorage.destroySession(session);
}
