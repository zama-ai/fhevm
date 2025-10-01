import {
  getStripeCustomerId,
  getStripeCustomerIdFromCache,
} from "./stripeApis";

/**
 * Returns a unified customer ID, using cache if possible.
 */
export function getUnifiedCustomerIdCached(
  authUser: any,
  email?: string
): string | undefined {
  return (
    getStripeCustomerIdFromCache(email || authUser?.email) ||
    authUser?.sub ||
    authUser?.user_id ||
    authUser?.id ||
    email
  );
}

/**
 * Returns a unified customer ID, possibly asynchronously.
 * This is a way to get
 */
export async function getUnifiedCustomerId(
  authUser: any,
  email?: string
): Promise<string | undefined> {
  if (process.env.APP_PAYMENT_PROVIDER === "stripe") {
    // in stripe implementation
    // we use customer id from stripe as the mapping to Moesif userid.
    return getStripeCustomerId(email || authUser?.email);
  }

  // for custom billing set up we can use the user id provided by the
  // authentication provider as userid
  return authUser?.sub || authUser?.user_id || authUser?.id || email;
}
