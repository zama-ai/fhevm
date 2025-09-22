const moesif = require("moesif-nodejs");

const moesifManagementToken = process.env.MOESIF_MANAGEMENT_TOKEN;
const moesifApiEndPoint = "https://api.moesif.com";

const moesifMiddleware = moesif({
  applicationId: process.env.MOESIF_APPLICATION_ID,

  identifyUser: function (req, _res) {
    return req.user ? req.user.id : undefined;
  },
});

function syncToMoesif({ companyId, userId, email }) {
  if (companyId) {
    var company = {
      companyId: companyId,
      metadata: {
        // feel free to add additonal profile data here.
      },
    };
    moesifMiddleware.updateCompany(company);
  }
  if (userId) {
    var user = {
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
function sendSubscriptionToMoesif({
  companyId,
  subscriptionId,
  planId,
  priceId,
  currentPeriodStart,
  currentPeriodEnd,
  metadata,
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
      "X-Moesif-Application-Id": process.env.MOESIF_APPLICATION_ID,
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

function getPlansFromMoesif() {
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

function getCompany({ companyId }) {
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

function getUser({ userId }) {
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

function extractSubscriptionsFromCompanyObject(companyObject) {
  // for MOESIF_MONETIZATION_VERSION V2, subscriptions are directly under companyObject
  if (companyObject?.subscriptions) {
    return companyObject?.subscriptions;
  } else if (companyObject?.metadata?.stripe?.subscription) {
    // for MOESIF_MONETIZATION_VERSION V2, the subscription is under metadata billing provider
    return [companyObject?.metadata?.stripe?.subscription];
  }
  return null;
}

function getSubscriptionsForCompanyId({ companyId }) {
  return getCompany({ companyId }).then((companyObject) => {
    return extractSubscriptionsFromCompanyObject(companyObject);
  });
}

// it simply gets the userObject from Moesif.
// subscriptions are under "company.subscriptions"
function getSubscriptionsForUserId({ userId }) {
  return getUser({ userId }).then((userObject) => {
    return extractSubscriptionsFromCompanyObject(userObject?.company);
  });
}

function getSubscriptionForUserEmail({ email }) {
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
      return res.json();
    })
    .then((data) => {
      console.log(`got moesif user object for ${email}`);
      console.log(JSON.stringify(data));
      // const exampleReturnValue = {
      //   "hits": {
      //     "hits": [
      //       {
      //         "_source": {
      //           "company_id": "sub_1OUHDjKjbeAxuumJy6ko1gXu",
      //           "name": "Foo Bar",
      //           "company": {
      //             "subscriptions": [
      //               {
      //                 "cancel_time": null,
      //                 "metadata": {},
      //                 "company_id": "sub_1OUHDjKjbeAxuumJy6ko1gXu",
      //                 "items": [
      //                   {
      //                     "subscription_item_id": "si_PIszPQaPC3zIZL",
      //                     "price": {
      //                       "tax_behavior": "unspecified",
      //                       "period": 1,
      //                       "metadata": {},
      //                       "created_at": "2024-01-02T18:55:23.000",
      //                       "price_in_decimal": 50,
      //                       "provider": "stripe",
      //                       "pricing_model": "flat",
      //                       "usage_aggregator": null,
      //                       "currency": "USD",
      //                       "period_units": "M",
      //                       "id": "price_1OUD7XKjbeAxuumJwK5agr87",
      //                       "plan_id": "prod_PIolHSJQuNP5Wm",
      //                       "status": "active"
      //                     },
      //                     "price_id": "price_1OUD7XKjbeAxuumJwK5agr87",
      //                     "created_at": "2024-01-02T23:18:03.000Z",
      //                     "plan": {
      //                       "metadata": {},
      //                       "provider": "stripe",
      //                       "created_at": "2024-01-02T18:55:23.000",
      //                       "id": "prod_PIolHSJQuNP5Wm",
      //                       "status": "active"
      //                     },
      //                     "plan_id": "prod_PIolHSJQuNP5Wm",
      //                     "status": "active"
      //                   },
      //                   {
      //                     "subscription_item_id": "si_PIwfDNLxCl3oxz",
      //                     "price": {
      //                       "tax_behavior": "unspecified",
      //                       "period": 1,
      //                       "metadata": {},
      //                       "created_at": "2024-01-02T22:03:55.000",
      //                       "price_in_decimal": 1,
      //                       "provider": "stripe",
      //                       "pricing_model": "per_unit",
      //                       "usage_aggregator": "sum",
      //                       "currency": "USD",
      //                       "period_units": "M",
      //                       "id": "price_1OUG3zKjbeAxuumJ3Zmidpum",
      //                       "plan_id": "prod_PIolHSJQuNP5Wm",
      //                       "status": "active"
      //                     },
      //                     "price_id": "price_1OUG3zKjbeAxuumJ3Zmidpum",
      //                     "created_at": "2024-01-03T03:06:09.000Z",
      //                     "plan": {
      //                       "metadata": {},
      //                       "provider": "stripe",
      //                       "created_at": "2024-01-02T22:03:55.000",
      //                       "id": "prod_PIolHSJQuNP5Wm",
      //                       "status": "active"
      //                     },
      //                     "plan_id": "prod_PIolHSJQuNP5Wm",
      //                     "status": "active"
      //                   }
      //                 ],
      //                 "app_id": "660:387",
      //                 "current_period_start": "2024-03-02T23:18:03.000Z",
      //                 "status": "active",
      //                 "start_date": "2024-01-02T23:18:03.000Z"
      //               }
      //             ]
      //           },
      //           "identified_user_id": "cus_PIsz7TqLLrheqX",
      //           "email": "foo_bar@moesif.com"
      //         },
      //         "_id": "cus_PIsz7TqLLrheqX"
      //       }
      //     ],
      //     "total": 1
      //   },
      // }

      const firstUserObject = data?.hits?.hits?.[0]?._source;
      return extractSubscriptionsFromCompanyObject(firstUserObject?.company);
    });
}

function getInfoForEmbeddedWorkspaces({ userId, workspaceId }) {
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

module.exports = {
  syncToMoesif,
  getPlansFromMoesif,
  getInfoForEmbeddedWorkspaces,
  getSubscriptionsForCompanyId,
  getSubscriptionsForUserId,
  getSubscriptionForUserEmail,
  sendSubscriptionToMoesif,
};
