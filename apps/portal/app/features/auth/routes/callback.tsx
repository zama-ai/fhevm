import { redirect, type LoaderFunctionArgs } from "react-router";
import {
  exchangeAuthCodeForTokens,
  getUserInfo,
} from "~/features/auth/auth0.server";
import { createUserSession } from "~/features/auth/session.server";

export async function loader({ request }: LoaderFunctionArgs) {
  const url = new URL(request.url);
  const code = url.searchParams.get("code");
  const error = url.searchParams.get("error");
  const errorDescription = url.searchParams.get("error_description");

  if (error) {
    console.error("Auth0 error:", error, errorDescription);
    return redirect(`/?error=${encodeURIComponent(errorDescription || error)}`);
  }

  if (!code) {
    return redirect("/?error=no_code");
  }

  try {
    // Exchange code for tokens
    const tokens = await exchangeAuthCodeForTokens(code, request);

    // Get user info
    const userInfo = await getUserInfo(tokens.access_token);

    // Create session and redirect
    return createUserSession(userInfo, "/dashboard");
  } catch (error) {
    console.error("Authentication error:", error);
    return redirect("/?error=authentication_failed");
  }
}

export default function Callback() {
  return (
    <div className="flex items-center justify-center min-h-screen">
      <div className="text-center">
        <h2 className="text-2xl font-bold mb-4">Authenticating...</h2>
        <p className="text-gray-600">Please wait while we log you in.</p>
      </div>
    </div>
  );
}
