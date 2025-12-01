import { http, HttpResponse } from "msw";

const AUTH0_DOMAIN = process.env.AUTH0_DOMAIN || "test.auth0.com";

// Store for Auth0 state between authorize and token exchange
const authState = new Map();

export const handlers = [
  // Authorization endpoint - browser redirect
  http.get(`https://${AUTH0_DOMAIN}/authorize`, ({ request }) => {
    const url = new URL(request.url);
    const redirectUri = url.searchParams.get("redirect_uri");
    const state = url.searchParams.get("state");
    const screenHint = url.searchParams.get("screen_hint");

    if (!redirectUri || !state) {
      return new HttpResponse("Missing required parameters", { status: 400 });
    }

    // Cache redirect context for debugging if needed
    authState.set(state, {
      redirectUri,
      screenHint: screenHint || undefined,
    });

    // Immediately redirect back to application callback with mock code
    const callbackUrl = new URL(redirectUri);
    callbackUrl.searchParams.set("code", "mock_auth_code");
    callbackUrl.searchParams.set("state", state);

    const body = `<!DOCTYPE html><html><head><meta charset="utf-8" /><script>window.location.href = "${callbackUrl.toString()}";</script></head><body></body></html>`;

    return new HttpResponse(body, {
      status: 200,
      headers: {
        "Content-Type": "text/html",
      },
    });
  }),

  // Token endpoint - backend exchange
  http.post(`https://${AUTH0_DOMAIN}/oauth/token`, async ({ request }) => {
    const body = await request.json();
    const { code } = body;

    if (code === "mock_auth_code") {
      return HttpResponse.json({
        access_token: "mock_access_token",
        id_token: "mock_id_token",
        token_type: "Bearer",
        expires_in: 3600,
      });
    }

    return HttpResponse.json(
      {
        error: "invalid_grant",
        error_description: "Invalid authorization code",
      },
      { status: 400 }
    );
  }),

  // Userinfo endpoint - backend fetch after token exchange
  http.get(`https://${AUTH0_DOMAIN}/userinfo`, ({ request }) => {
    const authHeader = request.headers.get("authorization");

    if (authHeader === "Bearer mock_access_token") {
      return HttpResponse.json({
        sub: "auth0|1234567890",
        name: "Test User",
        email: "test@example.com",
        picture: "https://ui-avatars.com/api/?name=Test+User",
        email_verified: true,
      });
    }

    return HttpResponse.json(
      {
        error: "invalid_token",
        error_description: "Invalid access token",
      },
      { status: 401 }
    );
  }),

  // Logout endpoint - ensure redirect back to app
  http.get(`https://${AUTH0_DOMAIN}/v2/logout`, ({ request }) => {
    const url = new URL(request.url);
    const returnTo =
      url.searchParams.get("returnTo") || "http://localhost:5173";

    const body = `<!DOCTYPE html><html><head><meta charset="utf-8" /><script>window.location.href = "${returnTo}";</script></head><body></body></html>`;

    return new HttpResponse(body, {
      status: 200,
      headers: {
        "Content-Type": "text/html",
      },
    });
  }),
];
