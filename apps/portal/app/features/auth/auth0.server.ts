export interface Auth0Config {
  domain: string;
  clientId: string;
  clientSecret: string;
  audience?: string;
}

export function getAuth0Config(): Auth0Config {
  const domain = process.env.AUTH0_DOMAIN;
  const clientId = process.env.AUTH0_CLIENT_ID;
  const clientSecret = process.env.AUTH0_CLIENT_SECRET;
  const audience = process.env.AUTH0_AUDIENCE;

  if (!domain || !clientId || !clientSecret) {
    throw new Error("Missing Auth0 configuration in environment variables");
  }

  return {
    domain,
    clientId,
    clientSecret,
    audience,
  };
}

export function getAuth0Urls(request: Request) {
  const config = getAuth0Config();
  const url = new URL(request.url);
  const redirectUri = `${url.protocol}//${url.host}/auth/callback`;

  return {
    authorizationUrl: `https://${config.domain}/authorize`,
    tokenUrl: `https://${config.domain}/oauth/token`,
    userInfoUrl: `https://${config.domain}/userinfo`,
    logoutUrl: `https://${config.domain}/v2/logout`,
    redirectUri,
  };
}

export async function exchangeAuthCodeForTokens(
  code: string,
  request: Request
) {
  const config = getAuth0Config();
  const { tokenUrl, redirectUri } = getAuth0Urls(request);

  const response = await fetch(tokenUrl, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      grant_type: "authorization_code",
      client_id: config.clientId,
      client_secret: config.clientSecret,
      code,
      redirect_uri: redirectUri,
    }),
  });

  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(
      `Failed to exchange auth code for tokens: ${
        errorData.error_description || response.statusText
      }`
    );
  }

  return response.json();
}

export async function getUserInfo(accessToken: string) {
  const config = getAuth0Config();
  const userInfoUrl = `https://${config.domain}/userinfo`;

  const response = await fetch(userInfoUrl, {
    method: "GET",
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  });

  if (!response.ok) {
    const errorData = await response.json();
    throw new Error(
      `Failed to fetch user info: ${errorData.error || response.statusText}`
    );
  }

  return response.json();
}

export function getLogoutUrl(request: Request) {
  const config = getAuth0Config();
  const url = new URL(request.url);
  const returnTo = `${url.protocol}//${url.host}`;

  return `https://${config.domain}/v2/logout?client_id=${
    config.clientId
  }&returnTo=${encodeURIComponent(returnTo)}`;
}

export function redirectToAuth0(
  request: Request,
  options?: { isSignUp?: boolean }
) {
  const config = getAuth0Config();
  const { authorizationUrl, redirectUri } = getAuth0Urls(request);

  const state = crypto.randomUUID();
  const nonce = crypto.randomUUID();

  const params = new URLSearchParams({
    response_type: "code",
    client_id: config.clientId,
    redirect_uri: redirectUri,
    scope: "openid profile email",
    state,
    nonce,
    ...(options?.isSignUp && { screen_hint: "signup" }),
    ...(config.audience && { audience: config.audience }),
  });

  return `${authorizationUrl}?${params.toString()}`;
}
