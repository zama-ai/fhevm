const crypto = require("crypto");

function getOneMonthFromNowISO() {
  const now = new Date(); // Get the current date and time
  now.setMonth(now.getMonth() + 1); // Add one calendar month
  return now.toISOString(); // Convert to ISO 8601 format
}

/* Interface for Billing Provider */
class BillingProvider {
  verifyPurchaseAndCreateSubscription(req, data) {
    // use the request info to verify the purchase after checkout
    // but generally your billing provider should verify the subscription
    // and return the subscription object.
    // below is a fake subscription generated on the fly.

    const subscription = {
      id: crypto.randomUUID(),
      plan_id: data?.plan_id,
      price_id: data?.price_id,
      ...data,
      current_period_start: new Date().toISOString(),
      current_period_end: getOneMonthFromNowISO(),
    };

    return {
      subscription,
    };
    // throw new Error('Verify Purchase Not Implemented.');
  }
}

module.exports = {
  BillingProvider,
};
