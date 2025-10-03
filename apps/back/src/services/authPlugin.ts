import { Request, Response, NextFunction } from "express";
import jwt, { JwtPayload } from "jsonwebtoken";
import jwksRsa from "jwks-rsa";
import { getLogger } from "../common/logger.context";
import { Span, withSpan } from "../decorators/span";

// Extend Express Request type to include user
export interface UserRequest extends Request {
  user?: any;
}

// In order auth0 setup you may have set up symmetric key.
export const verifyTokenJWTSymmetricKey = withSpan(
  "verifyTokenJWTSymmetricKey",
  (req: UserRequest, res: Response, next: NextFunction) => {
    const logger = getLogger().child({ method: "verifyTokenJWTSymmetricKey" });
    const token = req.headers.authorization;

    if (!process.env.PORTAL_JWT_CLIENT_SECRET) {
      logger.error(`missing PORTAL_JWT_CLIENT_SECRET`);
      throw new Error(
        "using JWT symmetric key verification, the PORTAL_JWT_CLIENT_SECRET must be provided"
      );
    }

    if (!token) {
      logger.warn(`No token provided`);
      return res.status(401).json({ error: "No token provided" });
    }

    try {
      const decoded = jwt.verify(
        token,
        process.env.PORTAL_JWT_CLIENT_SECRET as string
      ) as JwtPayload;
      req.user = decoded;
      // the sub is usually the user id in tokens.
      req.user.id = decoded.id || decoded.sub;

      if (!decoded.email) {
        logger.warn(`Unauthorized`);
        return res.status(401).json({
          message:
            "the jwt token provided do not have an email claim, are you using idToken? If using accessToken be sure to configure in your identify provider to add email in the claim.",
        });
      }

      next();
    } catch (error) {
      logger.warn(`Invalid ID token`);
      return res.status(403).json({ error: "Invalid ID token" });
    }
  }
);

export const verifyTokenJWTPublicKey = withSpan(
  "verifyTokenJWTPublicKey",
  async (req: UserRequest, res: Response, next: NextFunction) => {
    const logger = getLogger().child({ method: "verifyTokenJWTPublicKey" });
    const authHeader = req.headers.authorization;
    const token = authHeader?.split(" ")[1]; // Extract Bearer token

    if (!token) {
      logger.warn(`No token`);
      return res
        .status(401)
        .json({ message: "Authorization token is required" });
    }

    const provider = process.env.AUTH_PROVIDER;
    let issuer: string;
    let jwksClient: jwksRsa.JwksClient;

    if (provider === "Auth0") {
      issuer = `https://${process.env.AUTH0_DOMAIN}/`;
      jwksClient = jwksRsa({
        cache: true,
        rateLimit: true,
        jwksRequestsPerMinute: 10,
        jwksUri: `https://${process.env.AUTH0_DOMAIN}/.well-known/jwks.json`,
      });
    } else if (provider === "Okta") {
      issuer = `${process.env.OKTA_ORG_URL}/oauth2/default`;
      jwksClient = jwksRsa({
        cache: true,
        rateLimit: true,
        jwksRequestsPerMinute: 10,
        jwksUri: `${issuer}/v1/keys`,
      });
    } else {
      logger.warn(`Unsupported authentication provider`);
      throw new Error("Unsupported authentication provider");
    }

    const decodedHeader = jwt.decode(token, { complete: true }) as {
      header: { kid: string };
    } | null;
    const kid = decodedHeader?.header?.kid;

    if (!kid) {
      logger.warn(`Invalid token header`);
      return res.status(401).json({ message: "Invalid token header" });
    }

    try {
      // Get the signing key from the JWKS
      const key = await jwksClient.getSigningKey(kid);
      const publicKey = key.getPublicKey();

      // Verify the token
      const verifiedClaims = jwt.verify(token, publicKey, {
        algorithms: ["RS256"],
        issuer,
      }) as JwtPayload;

      req.user = verifiedClaims;

      if (!verifiedClaims.email) {
        return res.status(401).json({
          message:
            "the jwt token provided do not have an email claim, are you using idToken? If using accessToken be sure to configure in your identify provider to add email in the claim.",
        });
      }

      req.user.id = verifiedClaims.id || verifiedClaims.sub;
      next();
    } catch (err) {
      logger.warn("error from jwt verify", err);
      return res.status(401).json({ message: "Invalid token", error: err });
    }
  }
);

export const skipAuthCheck = withSpan(
  "skipAuthCheck",
  (req: UserRequest, _res: Response, next: NextFunction) => {
    const logger = getLogger().child({ method: "skipAuthCheck" });
    logger.info(
      "skip authentication check in dev portal apis, enable by configure AUTH_PROVIDER in .env."
    );
    req.user = {};
    next();
  }
);

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

export const authMiddleware = getFinalChecker();
