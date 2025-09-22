const jwt = require("jsonwebtoken");
const jwksRsa = require("jwks-rsa");

// In order auth0 setup you may have set up symmetric key.
const verifyTokenJWTSymmetricKey = (req, res, next) => {
  const token = req.headers.authorization;

  if (!process.env.PORTAL_JWT_CLIENT_SECRET) {
    throw new Error(
      "using JWT symmetric key verification, the PORTAL_JWT_CLIENT_SECRET must be provided"
    );
  }

  if (!token) {
    return res.status(401).json({ error: "No token provided" });
  }

  try {
    const decoded = jwt.verify(idToken, process.env.PORTAL_JWT_CLIENT_SECRET);
    req.user = decoded;
    // the sub is usually the user id in tokens.
    req.user.id = decoded.id || decoded.sub;

    if (!decoded.email) {
      return res.status(401).json({
        message:
          "the jwt token provided do not have an email claim, are you using idToken? If using accessToken be sure to configure in your identify provider to add email in the claim.",
      });
    }

    next();
  } catch (error) {
    return res.status(403).json({ error: "Invalid ID token" });
  }
};

const verifyTokenJWTPublicKey = async (req, res, next) => {
  const token = req.headers.authorization?.split(" ")[1]; // Extract Bearer token

  if (!token) {
    return res.status(401).json({ message: "Authorization token is required" });
  }

  const provider = process.env.AUTH_PROVIDER;
  let issuer;
  let jwksClient;

  if (provider === "Auth0") {
    issuer = `https://${process.env.AUTH0_DOMAIN}/`;
    // jwksClient.jwksUri = `https://${process.env.AUTH0_DOMAIN}/.well-known/jwks.json`;

    jwksClient = jwksRsa({
      cache: true, // Enable caching
      rateLimit: true, // Prevent excessive requests
      jwksRequestsPerMinute: 10, // Limit requests per minute
      jwksUri: `https://${process.env.AUTH0_DOMAIN}/.well-known/jwks.json`,
    });
  } else if (provider === "Okta") {
    issuer = `${process.env.OKTA_ORG_URL}/oauth2/default`;
    // jwksClient.jwksUri = `${issuer}/v1/keys`;
    jwksClient = jwksRsa({
      cache: true, // Enable caching
      rateLimit: true, // Prevent excessive requests
      jwksRequestsPerMinute: 10, // Limit requests per minute
      jwksUri: `${issuer}/v1/keys`,
    });
  } else {
    throw new Error("Unsupported authentication provider");
  }

  const decodedHeader = jwt.decode(token, { complete: true });
  const kid = decodedHeader.header.kid;

  try {
    // Get the signing key from the JWKS
    const key = await jwksClient.getSigningKey(kid);
    const publicKey = key.getPublicKey(); // This is the public key used for verification

    // Verify the toke
    const verifiedClaims = jwt.verify(token, publicKey, {
      algorithms: ["RS256"],
      issuer,
    });
    req.user = verifiedClaims;
    // the sub is usually the user id in tokens.

    if (!verifiedClaims.email) {
      return res.status(401).json({
        message:
          "the jwt token provided do not have an email claim, are you using idToken? If using accessToken be sure to configure in your identify provider to add email in the claim.",
      });
    }

    req.user.id = verifiedClaims.id || verifiedClaims.sub;
    next();
  } catch (err) {
    console.error("error from jwt verify", err);
    return res.status(401).json({ message: "Invalid token", error: err });
  }
};

const skipAuthCheck = (req, res, next) => {
  console.log(
    "skip authentication check in dev portal apis, enable by configure AUTH_PROVIDER in .env."
  );
  req.user = {};
  next();
};

function getFinalChecker() {
  switch (process.env.AUTH_PROVIDER) {
    case "Okta":
      if (!process.env.OKTA_ORG_URL) {
        throw new Error(
          "using Okta provider, the OKTA_ORG_URL must be provided in .env"
        );
      }
      return verifyTokenJWTPublicKey;
    case "Auth0":
      if (!process.env.AUTH0_DOMAIN) {
        throw new Error(
          "using Auth0 provider, the AUTH0_DOMAIN must be provided in .env"
        );
      }
      return verifyTokenJWTPublicKey;
    default:
      throw Error(
        "Unsupported Auth Provider for dev-portal-api, please check AUTH_PROVIDER configuration in /my-dev-portal-api/.env"
      );
  }
}

module.exports = {
  verifyTokenJWTSymmetricKey,
  verifyTokenJWTPublicKey,
  skipAuthCheck,
  authMiddleware: getFinalChecker(),
};
