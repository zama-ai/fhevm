import isNil from "lodash/isNil";

export function formatPrice(priceInDecimal = 0) {
  if (isNil(priceInDecimal)) {
    return "";
  }

  const priceInDollars = Number(priceInDecimal) / 100;

  // Format the price as a currency string with up to 10 decimal places
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 0, // Minimum number of decimal places
    maximumFractionDigits: 10, // Maximum number of decimal places
  }).format(priceInDollars);
}

export function formatPeriod(periodUnits, period) {
  switch (periodUnits) {
    case "y":
      return "yearly";
    case "d":
      return "daily";
    case "M":
    default:
      return "monthly";
  }
}

export function formatIsoTimestamp(isoString) {
  // Create a new Date object from the ISO string
  try {
    const date = new Date(isoString);

    // Format the date to a human-readable string using the browser's locale
    return date.toLocaleString(undefined, {
      year: "numeric",
      month: "long",
      day: "numeric",
      hour: "2-digit",
      minute: "2-digit",
      second: "2-digit",
      hour12: true, // Use 12-hour format
    });
  } catch (err) {
    return "";
  }
}

let stripeCustomerCache = {};

export async function moesifIdentifyUserFrontEndIfPossible(idToken, user) {
  // we are using stripe customer id
  // and as the user_id mapped to moesif users.
  // See DATA_MODEL.md
  if (!window?.moesif || !idToken) {
    return;
  }

  console.log(
    "try to identifyUser for moesif using stripe customer id if exists"
  );

  if (import.meta.env.REACT_APP_PAYMENT_PROVIDER === "custom") {
    return user?.sub || user?.user_id || user?.id;
  }

  if (stripeCustomerCache[idToken]) {
    const stripeCustomerObject = stripeCustomerCache[idToken];
    window.moesif.identifyUser(stripeCustomerObject.id, {
      ...stripeCustomerObject,
    });
  } else {
    fetch(`${import.meta.env.REACT_APP_DEV_PORTAL_API_SERVER}/stripe/customer`, {
      method: "GET",
      headers: {
        Authorization: `Bearer ${idToken}`,
      },
    })
      .then((res) => res.json())
      .then((stripeCustomerObject) => {
        if (stripeCustomerObject && stripeCustomerObject.id) {
          window.moesif.identifyUser(stripeCustomerObject.id, {
            ...stripeCustomerObject,
          });
          stripeCustomerCache[idToken] = stripeCustomerObject;
        }
      });
  }
}
