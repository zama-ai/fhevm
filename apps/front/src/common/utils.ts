import isNil from "lodash/isNil";

import config from "../config";

export function formatPrice(
  priceInDecimal: string | number = 0,
  currency = "USD"
) {
  if (isNil(priceInDecimal)) {
    return "";
  }

  const price =
    typeof priceInDecimal === "string"
      ? parseFloat(priceInDecimal)
      : priceInDecimal;

  // Format the price as a currency string with up to 10 decimal places
  return new Intl.NumberFormat("en-US", {
    style: "currency",
    currency,
    minimumFractionDigits: 0, // Minimum number of decimal places
    maximumFractionDigits: 10, // Maximum number of decimal places
  }).format(price);
}

export function formatPeriod(periodUnits: string, _period: number) {
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

export function formatIsoTimestamp(isoString: string) {
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
  } catch (_err) {
    return "";
  }
}

interface UserMetadata {
  email?: string;
  firstName?: string;
  lastName?: string;
  [key: string]: any;
}

let stripeCustomerCache: Record<string, UserMetadata> = {};

export async function moesifIdentifyUserFrontEndIfPossible(
  idToken?: string,
  user?: UserMetadata
) {
  // we are using stripe customer id
  // and as the user_id mapped to moesif users.
  // See DATA_MODEL.md
  if (!window?.moesif || !idToken) {
    return;
  }

  console.log(
    "try to identifyUser for moesif using stripe customer id if exists"
  );

  if (config.paymentProvider === "custom") {
    return user?.sub || user?.user_id || user?.id;
  }

  if (stripeCustomerCache[idToken]) {
    const stripeCustomerObject = stripeCustomerCache[idToken];
    window.moesif.identifyUser(stripeCustomerObject.id, {
      ...stripeCustomerObject,
    });
  } else {
    fetch(`${config.devPortalApiServer}/stripe/customer`, {
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
