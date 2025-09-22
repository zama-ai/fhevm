const {
  getStripeCustomerId,
  getStripeCustomerIdFromCache,
} = require("./stripeApis");

function getUnifiedCustomerIdCached(authUser, email) {
  return (
    getStripeCustomerIdFromCache(email || authUser?.email) ||
    authUser?.sub ||
    authUser?.user_id ||
    authUser?.id ||
    email
  );
}

async function getUnifiedCustomerId(authUser, email) {
  // this is a way to get
  if (process.env.APP_PAYMENT_PROVIDER === "stripe") {
    // in stripe implementation
    // we use customer id from stripe as the mapping to Moesif userid.
    return getStripeCustomerId(email || authUser?.email);
  }

  // for custom billing set up we can use the user id provided by the
  // authentication provider as userid
  return authUser?.sub || authUser?.user_id || authUser?.id || email;
}

module.exports = {
  getUnifiedCustomerIdCached,
  getUnifiedCustomerId,
};
