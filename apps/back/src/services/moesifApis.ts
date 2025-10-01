import moesif from "moesif-nodejs";

const moesifManagementToken = process.env.MOESIF_MANAGEMENT_TOKEN;
const moesifApiEndPoint = "https://api.moesif.com";

const applicationId = process.env.MOESIF_APPLICATION_ID;
if (!applicationId) {
  throw new Error("No MOESIF_APPLICATION_ID found.");
}

const moesifMiddleware = moesif({
  applicationId,

  identifyUser: function (req: any, _res: any) {
    return req.user ? req.user.id : undefined;
  },
});

/**
 * Syncs user and company data to Moesif.
 */
export function syncToMoesif({
  companyId,
  userId,
  email,
}: {
  companyId?: string;
  userId?: string;
  email?: string;
}) {
  if (companyId) {
    const company = {
      companyId: companyId,
      metadata: {
        // feel free to add additonal profile data here.
      },
    };
    moesifMiddleware.updateCompany(company);
  }
  if (userId) {
    const user = {
      userId: userId,
      companyId: companyId,
      metadata: {
        // feel free to add additional profile data here.
        email: email,
      },
    };
    moesifMiddleware.updateUser(user);
  }
}

// for Stripe, if you set up webhook in Stripe, below is NOT needed
// since Moesif automatically listen to Stripe's webhook for Subscription updates.
// but if you are building a custom billing provider, you must send
// the subscription data to Moesif.
export function sendSubscriptionToMoesif({
  companyId,
  subscriptionId,
  planId,
  priceId,
  currentPeriodStart,
  currentPeriodEnd,
  metadata,
}: {
  companyId: string;
  subscriptionId: string;
  planId?: string;
  priceId?: string;
  currentPeriodStart: string;
  currentPeriodEnd: string;
  metadata?: Record<string, any>;
}) {
  const payload = {
    subscription_id: subscriptionId,
    company_id: companyId,
    current_period_start: currentPeriodStart,
    current_period_end: currentPeriodEnd,
    status: "active",
    items: [
      {
        plan_id: planId,
        price_id: priceId,
      },
    ],
    metadata: {
      // this is custom data you would like to be available.
      subscription_type: "PAYG",
      subscription_tier: "Pro",
      ...metadata,
    },
  };

  console.log(
    "about to send subscription data to moesif" +
      JSON.stringify(payload, null, "  ")
  );

  return fetch(`https://api.moesif.net/v1/subscriptions`, {
    method: "POST",
    headers: {
      "X-Moesif-Application-Id": applicationId!,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  })
    .then(async (res) => {
      if (res.ok) {
        console.log(
          `subscription sent to moesif successfully for ${companyId}`
        );
      } else {
        const text = await res.text();
        console.error("failed to log to moesif", text);
      }
    })
    .catch((err) => {
      console.error(`failed to send event to ${companyId}`, err);
    });
}

export function getPlansFromMoesif(): Promise<any> {
  return fetch(
    `https://api.moesif.com/v1/~/billing/catalog/plans?includes=prices&provider=${process.env.APP_PAYMENT_PROVIDER}`,
    {
      headers: {
        Authorization: `Bearer ${moesifManagementToken}`,
      },
    }
  )
    .then((res) => {
      if (!res.ok) {
        console.error("get plans from moesif not successful" + res.statusText);
      }
      return res;
    })
    .then((res) => res.json());
}

export function getCompany({ companyId }: { companyId: string }): Promise<any> {
  return fetch(
    `https://api.moesif.com/v1/search/~/companies/${encodeURIComponent(
      companyId
    )}`,
    {
      headers: {
        Authorization: `Bearer ${moesifManagementToken}`,
      },
    }
  )
    .then((res) => {
      if (!res.ok) {
        console.error(
          "get company from moesif not successful" + res.statusText
        );
      }
      return res;
    })
    .then((res) => res.json());
}

export function getUser({ userId }: { userId: string }): Promise<any> {
  return fetch(
    `https://api.moesif.com/v1/search/~/users/${encodeURIComponent(userId)}`,
    {
      headers: {
        Authorization: `Bearer ${moesifManagementToken}`,
      },
    }
  )
    .then((res) => {
      if (!res.ok) {
        console.error("get user from moesif not successful" + res.statusText);
      }
      return res;
    })
    .then((res) => res.json());
}

/**
 * Extracts subscriptions from a company object.
 */
export function extractSubscriptionsFromCompanyObject(companyObject: any): any {
  // for MOESIF_MONETIZATION_VERSION V2, subscriptions are directly under companyObject
  if (companyObject?.subscriptions) {
    return companyObject?.subscriptions;
  } else if (companyObject?.metadata?.stripe?.subscription) {
    // for MOESIF_MONETIZATION_VERSION V2, the subscription is under metadata billing provider
    return [companyObject?.metadata?.stripe?.subscription];
  }
  return null;
}

export function getSubscriptionsForCompanyId({
  companyId,
}: {
  companyId: string;
}): Promise<any> {
  return getCompany({ companyId }).then((companyObject) => {
    return extractSubscriptionsFromCompanyObject(companyObject);
  });
}

// it simply gets the userObject from Moesif.
// subscriptions are under "company.subscriptions"
export function getSubscriptionsForUserId({
  userId,
}: {
  userId: string;
}): Promise<any> {
  return getUser({ userId }).then((userObject) => {
    return extractSubscriptionsFromCompanyObject(userObject?.company);
  });
}

type Hit = {
  _id: string;
  _source: {
    user_id: string;
    name: string;
    email: string;
    identified_user_id: string;
  } & (
    | {
        company_id: string;
        company: Company;
      }
    | { company_id?: never; company?: never }
  );
};

export type Subscription = {
  metadata: Record<string, any>;
  subscription_id: string;
  provider: "stripe";
  payment_status: "complete" | "incomplete";
  status: "active" | "cancelled";
};

type Company = {
  subscriptions: Subscription[];
};

export function getSubscriptionForUserEmail({
  email,
}: {
  email: string;
}): Promise<any> {
  const query = {
    query: { term: { "email.raw": email } },
    size: 50,
    _source: [
      "user_id",
      "identified_user_id",
      "email",
      "company_id",
      "company.subscriptions",
      "name",
    ],
  };

  return fetch(`https://api.moesif.com/v1/search/~/search/users`, {
    method: "POST",
    headers: {
      Authorization: `Bearer ${moesifManagementToken}`,
      "Content-Type": "application/json",
    },
    body: JSON.stringify(query),
  })
    .then(async (res) => {
      if (!res.ok) {
        const body = await res.json();
        console.log("error fetching this");
        throw new Error(
          `Error fetching subscriptions: ${res.status} ${JSON.stringify(body)}`
        );
      }
      return res.json() as Promise<{ hits: { hits: Hit[] } }>;
    })
    .then((data) => {
      console.log(`got moesif user object for ${email}`);
      console.log(JSON.stringify(data));
      // const exampleReturnValue = { ... }
      const firstUserObject = data?.hits?.hits?.[0]?._source;
      return extractSubscriptionsFromCompanyObject(firstUserObject?.company);
    });
}

export function getInfoForEmbeddedWorkspaces({
  userId,
  workspaceId,
}: {
  userId: string;
  workspaceId: string;
}): Promise<any> {
  const templateData = {
    template: {
      values: {
        user_id: userId,
      },
    },
  };

  // Set your desired expiration for the generated workspace token.
  // Moesif's recommendation is to match or be larger than your user's session time while keeping time period less than 30 days.
  const tomorrow = new Date();
  tomorrow.setDate(tomorrow.getDate() + 7);
  const expiration = tomorrow.toISOString();

  const moesif_url_live_event = `${moesifApiEndPoint}/v1/portal/~/workspaces/${workspaceId}/access_token?expiration=${expiration}`;

  return fetch(moesif_url_live_event, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${moesifManagementToken}`,
    },
    body: JSON.stringify(templateData),
  })
    .then((response) => {
      if (response.ok) {
        return response;
      } else {
        console.log("Api call to moesif not successful. server response is:");
        console.error(response.statusText);
        throw Error(response.statusText);
      }
    })
    .then((response) => {
      return response.json();
    });
}
