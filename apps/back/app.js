const express = require("express");
const path = require("path");
require("dotenv").config({ path: [".env", ".env.template"] });
const rateLimit = require("express-rate-limit");
const bodyParser = require("body-parser");
const moesif = require("moesif-nodejs");
const cors = require("cors");
const fetch = require("node-fetch");
const { Client } = require("@okta/okta-sdk-nodejs");

const {
  verifyStripeSession,
  getStripeCustomer,
  createStripeCheckoutSession,
} = require("./services/stripeApis");
const {
  syncToMoesif,
  getInfoForEmbeddedWorkspaces,
  getPlansFromMoesif,
  getSubscriptionsForUserId,
  sendSubscriptionToMoesif,
} = require("./services/moesifApis");

const { authMiddleware } = require("./services/authPlugin");

const { getApimProvisioningPlugin } = require("./config/pluginLoader");
const {
  getUnifiedCustomerId,
  getUnifiedCustomerIdCached,
} = require("./services/commonUtils");
const { BillingProvider } = require("./services/billingProvider");

const app = express();
app.use(express.static(path.join(__dirname)));
const port = parseInt(process.env.PORT ?? "3000", 10);

const moesifManagementToken = process.env.MOESIF_MANAGEMENT_TOKEN;
const templateWorkspaceIdLiveEvent =
  process.env.MOESIF_TEMPLATE_WORKSPACE_ID_LIVE_EVENT_LOG;
const templateWorkspaceIdTimeSeries =
  process.env.MOESIF_TEMPLATE_WORKSPACE_ID_TIME_SERIES;

var jsonParser = bodyParser.json();

if (!moesifManagementToken) {
  console.error(
    "No MOESIF_MANAGEMENT_TOKEN found. Please create an .env file with MOESIF_MANAGEMENT_TOKEN & MOESIF_TEMPLATE_WORKSPACE_ID."
  );
}

if (!templateWorkspaceIdLiveEvent) {
  console.error(
    "No MOESIF_TEMPLATE_WORKSPACE_ID found. Please create an .env file with MOESIF_MANAGEMENT_TOKEN & MOESIF_TEMPLATE_WORKSPACE_ID."
  );
}

const provisioningService = getApimProvisioningPlugin();

const customBillingProvider = new BillingProvider();

const moesifMiddleware = moesif({
  applicationId: process.env.MOESIF_APPLICATION_ID,

  identifyUser: function (req, _res) {
    return getUnifiedCustomerIdCached(req?.user);
  },
});

app.use(moesifMiddleware, cors());

app.post(
  "/create-stripe-checkout-session",
  rateLimit({
    windowMs: 1 * 60 * 1000, // 1 minute
    max: 10, // Limit each IP/user to 10 requests per windowMs
    message: "Too many checkout session requests, please try again later.",
  }),
  authMiddleware,
  async (req, res) => {
    const priceId = req.query?.price_id;
    const email = req.user?.email;
    const quantity = req.query?.quantity || undefined;

    console.log(
      `create-stripe-checkout-session called for ${email} priceId ${priceId} quantity ${quantity}`
    );

    if (!priceId) {
      return res.status(400).json({ message: "price_id is required" });
    }

    try {
      const session = await createStripeCheckoutSession(
        email,
        priceId,
        quantity,
        req?.user
      );
      console.log("got session back from stripe session");
      console.log(JSON.stringify(session));

      res.send({ clientSecret: session.client_secret });
    } catch (err) {
      console.error("Failed to create stripe checkout session", err);
      res.status(400).json({ message: "Error creating check out session" });
    }
  }
);

app.get("/plans", jsonParser, async (req, res) => {
  // if you created your "stripe" or "zoura" plans through moesif.
  // it is better
  getPlansFromMoesif()
    .then((result) => {
      res.status(200).json(result);
    })
    .catch((err) => {
      console.error("Error getting plans from Moesif", err);
      res.status(500).json({ message: "Error getting plans from Moesif" });
    });
});

app.get(
  "/subscriptions",
  rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100, // limit each IP to 100 requests per windowMs
    standardHeaders: true, // Return rate limit info in the `RateLimit-*` headers
    legacyHeaders: false, // Disable the `X-RateLimit-*` headers
  }),
  authMiddleware,
  jsonParser,
  async (req, res) => {
    // But in this project, we get from Moesif, because
    // Moesif syncs subscriptions from several billing providers.
    // - from moesif, you can get a list of associated subscriptions
    //   using companyId, userId or email.
    // - Your use case needs and data model/mapping inform the best approach. (See DATA_MODEL.md)
    //   for assumptions in this project.
    // - In this project, since Stripe customer id is mapped to user_id in Moesif,
    //   We use that as the user_id to fetch subscriptions.

    const sanitizedEmail = req.query.email.replace(/\n|\r/g, "");
    console.log("query email " + sanitizedEmail);
    console.log("verified email from claims " + req.user.email);
    const email = req.user?.email;

    let moesifUserId;
    try {
      moesifUserId = await getUnifiedCustomerId(req.user, email);
      // see DATA_MODEL.md regarding how customer ids are mapped.
      // please modify if you decides to use some other data mapping model.
      if (!moesifUserId) {
        return res.status(200).json([]);
      }

      const subscriptions = await getSubscriptionsForUserId({
        userId: moesifUserId,
      });

      console.log(
        "got subscriptions from moesif " + JSON.stringify(subscriptions)
      );
      res.status(200).json(subscriptions);
    } catch (err) {
      console.error(
        "Error getting subscription from moesif for " +
          email +
          " " +
          moesifUserId,
        err
      );
      res.status(404).json({ message: err.toString() });
    }
  }
);

app.post("/okta/register", jsonParser, async (req, res) => {
  try {
    const oktaClient = new Client({
      orgUrl: process.env.OKTA_DOMAIN,
      token: process.env.OKTA_API_TOKEN,
    });

    const { firstName, lastName, email, password } = req.body;

    const newUser = {
      profile: {
        firstName,
        lastName,
        email,
        login: email,
      },
      credentials: {
        password: {
          value: password,
        },
      },
    };

    const response = await fetch(`${process.env.OKTA_DOMAIN}/api/v1/users`, {
      method: "POST",
      headers: {
        Authorization: `SSWS ${process.env.OKTA_API_TOKEN}`,
        "Content-Type": "application/json",
      },
      body: JSON.stringify(newUser),
    });

    if (!response.ok) {
      throw new Error("Failed to create user");
    }

    const createdUser = await response.json();

    res
      .status(201)
      .json({ message: "User created successfully", user: createdUser });

    try {
      console.log(
        `URL = ${process.env.OKTA_DOMAIN}/api/v1/apps/${process.env.OKTA_APPLICATION_ID}/users/${createdUser.id}`
      );
      const assignUserResponse = await fetch(
        `${process.env.OKTA_DOMAIN}/api/v1/apps/${process.env.OKTA_APPLICATION_ID}/users/${createdUser.id}`,
        {
          method: "PUT",
          headers: {
            Authorization: `SSWS ${process.env.OKTA_API_TOKEN}`,
            "Content-Type": "application/json",
          },
        }
      );

      if (!assignUserResponse.ok) {
        throw new Error("Failed to assign user to application");
      }
      console.log("User assigned to application successfully.");
    } catch (error) {
      console.error("Failed to assign user to application:", error.message);
    }
  } catch (error) {
    console.error("Error creating user:", error);
    res.status(500).json({ message: "Failed to create user" });
  }
});

// This handles Provision after user success checked out from Stripe
// - syncing the Stripe ids to Moesif.
// - creates customers to API Management platform if need.
// - Please see DATA-MODEL.md see the assumptions and background on data mapping.
app.post(
  "/register/stripe/:checkout_session_id",
  rateLimit({
    windowMs: 5 * 60 * 1000, // 5 minutes
    max: 5, // limit each IP to 5 requests per windowMs
    standardHeaders: true, // Return rate limit info in the `RateLimit-*` headers
    legacyHeaders: false, // Disable the `X-RateLimit-*` headers
  }),
  authMiddleware,
  function (req, res) {
    const checkout_session_id = req.params.checkout_session_id;

    verifyStripeSession(checkout_session_id)
      .then(async (result) => {
        const stripeCheckOutSessionInfo = result;
        console.log("in stripe register");
        if (result.customer && result.subscription) {
          console.log("customer and subscription present");
          const email = result.customer_details.email;
          const stripe_customer_id = result.customer;
          const stripe_subscription_id = result.subscription;
          try {
            if (
              process.env.MOESIF_MONETIZATION_VERSION &&
              process.env.MOESIF_MONETIZATION_VERSION.toUpperCase() === "V1"
            ) {
              console.log("updating company and user with V1");
              // in v1, companyId and subscription id has one to one mapping.
              syncToMoesif({
                companyId: stripe_subscription_id,
                subscriptionId: stripe_subscription_id,
                userId: stripe_customer_id,
                email: email,
              });
            }
            // V2 as default
            else {
              console.log("updating company and user with V2");
              // assume you have one user per subscription
              // but if you have multiple users per each subscription
              // please check out https://www.moesif.com/docs/getting-started/overview/
              // for the different entities how they are related to each other.
              syncToMoesif({
                companyId: stripe_customer_id,
                subscriptionId: stripe_subscription_id,
                userId: stripe_customer_id,
                email: email,
              });
            }
          } catch (error) {
            console.error("Error updating user/company/sub:", error);
          }

          // Provision new user for access to API
          const user = await provisioningService.provisionUser(
            stripe_customer_id,
            email,
            stripe_subscription_id
          );
          console.log("provisioned user:", JSON.stringify(user));
        }
        // we still pass on result.
        console.log(JSON.stringify(stripeCheckOutSessionInfo));
        res.status(201).json(stripeCheckOutSessionInfo);
      })
      .catch((err) => {
        console.error("Error registering user", err);
        res.status(500).json({
          message: "Failed to provision user. " + err.toString(),
        });
      });
  }
);

// if you are using customer billing provider
// this should be triggered upon return from successful payment:
// - verify the purchase
// - create subscription object.
// - send the data to moesif.
// - provision the by calling API gateway plugin.
app.post(
  "/register/custom",
  rateLimit({
    windowMs: 60 * 1000, // 1 minute
    max: 5,
    standardHeaders: true, // Return rate limit info in the `RateLimit-*` headers
    legacyHeaders: false, // Disable the `X-RateLimit-*` headers
  }),
  authMiddleware,
  jsonParser,
  async function (req, res) {
    const customerId = await getUnifiedCustomerId(req.user);
    const email = req.user?.email;
    // verify plans and subscription using your custom billing provider.
    try {
      const { subscription } =
        await customBillingProvider.verifyPurchaseAndCreateSubscription(req, {
          user: req.user,
          ...req.body,
        });

      console.log("custom subscription created", subscription);

      syncToMoesif({
        companyId: customerId,
        userId: customerId,
        email: email,
      });

      sendSubscriptionToMoesif({
        companyId: customerId,
        subscriptionId: subscription.id,
        planId: subscription.plan_id,
        priceId: subscription.price_id,
        currentPeriodStart: subscription.current_period_start,
        currentPeriodEnd: subscription.current_period_end,
        metadata: {
          // additional metadata you might want add.
        },
      });

      const user = await provisioningService.provisionUser(
        customerId,
        email,
        subscription.id
      );
      res.status(201).json({ status: "provisioned" });
    } catch (err) {
      console.error("Error registering user", err);
      res.status(500).json({
        message: "Failed to provision user. " + err.toString(),
      });
    }
  }
);

app.get(
  "/stripe/customer",
  rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100, // limit each IP to 100 requests per windowMs
    message: "Too many requests, please try again later.",
  }),
  authMiddleware,
  function (req, res) {
    const email = req.user?.email;

    getStripeCustomer(email)
      .then((result) => {
        if (result.data && result.data[0]) {
          res.status(200).json(result.data[0]);
        } else {
          res.status(404).json("stripe customer not found");
        }
      })
      .catch((err) => {
        console.error("Error getting customer info from stripe", err);
        res.status(500).json({
          message: "Failed to retrieve customer info from stripe",
        });
      });
  }
);

app.post(
  "/create-key",
  rateLimit({
    windowMs: 60 * 1000, // 1 minute
    max: 5, // limit each IP to 5 requests per windowMs
    message: {
      message: "Too many requests to create API key, please try again later.",
    },
  }),
  authMiddleware,
  jsonParser,
  async function (req, res) {
    try {
      // if authentication used, email can come from idToken claims,
      // otherwise we use email from body.
      const email = req.user?.email;

      const customerId = await getUnifiedCustomerId(req.user, email);
      if (!customerId) {
        throw new Error(
          `Customer Id unknown. Ensure you're subscribed to a plan. If you just subscribed, try again.`
        );
      }

      // Provision new key for access to API
      const apiKey = await provisioningService.createApiKey(customerId, email);
      // Send the Tyk API key back as the response
      res.status(200).send({ apikey: apiKey });
    } catch (error) {
      console.error("Error creating key:", error);
      res.status(500).json({ message: "Failed to create key" });
    }
  }
);

app.get(
  "/embed-charts(/:authUserId)",
  rateLimit({
    windowMs: 15 * 60 * 1000, // 15 minutes
    max: 100, // limit each IP to 100 requests per windowMs
    standardHeaders: true, // Return rate limit info in the headers
    legacyHeaders: false, // Disable the `X-RateLimit-*` headers
  }),
  authMiddleware,
  async function (req, res) {
    // if authMiddleware is enabled, the data for user should come from the auth data.
    // otherwise use query param.
    const authUserId = req.user?.sub;
    const email = req.user?.email;

    // depends your data model (see assumptions in DATA_MODEL.md),
    // and if in your API gateway if you identifyUser using stripeCustomerId
    // or the userId from authorization provider.
    // Perhaps, you have your own userId for your own system.
    // the most important aspect is the user_id used in your identifyUser hook
    try {
      const customerId = await getUnifiedCustomerId(req.user, email);
      if (!customerId) {
        console.error("Customer Id not found when fetching for " + email);
      }

      const embedInfoArray = await Promise.all(
        [templateWorkspaceIdLiveEvent, templateWorkspaceIdTimeSeries].map(
          (workspaceId) =>
            getInfoForEmbeddedWorkspaces({
              workspaceId: workspaceId,
              userId: customerId,
            })
        )
      );
      res.status(200).json(embedInfoArray);
    } catch (err) {
      console.error("Error generating embedded templates:", err);
      res.status(500).json({ message: "Failed to retrieve embedded template" });
    }
  }
);

app.listen(port, () => {
  console.log(`My Dev Portal Backend is listening at http://localhost:${port}`);
});
