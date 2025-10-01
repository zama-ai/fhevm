import express, { Request, Response, NextFunction } from "express";
import path from "path";
import dotenv from "dotenv";
import bodyParser from "body-parser";
import moesif from "moesif-nodejs";
import cors from "cors";

// Load environment variables
dotenv.config({ path: [".env", ".env.template"] });

import {
  verifyStripeSession,
  getStripeCustomer,
  createStripeCheckoutSession,
} from "./services/stripeApis";
import {
  syncToMoesif,
  getInfoForEmbeddedWorkspaces,
  getPlansFromMoesif,
  getSubscriptionsForUserId,
  sendSubscriptionToMoesif,
} from "./services/moesifApis";

import { authMiddleware } from "./services/authPlugin";
import { getApimProvisioningPlugin } from "./config/pluginLoader";
import {
  getUnifiedCustomerId,
  getUnifiedCustomerIdCached,
} from "./services/commonUtils";
import { BillingProvider } from "./services/billingProvider";

const app = express();
app.use(express.static(path.join(__dirname)));
const port = parseInt(process.env.PORT ?? "3000", 10);

const moesifApplicationId = process.env.MOESIF_APPLICATION_ID;
const moesifManagementToken = process.env.MOESIF_MANAGEMENT_TOKEN;
const templateWorkspaceIdLiveEvent =
  process.env.MOESIF_TEMPLATE_WORKSPACE_ID_LIVE_EVENT_LOG;
const templateWorkspaceIdTimeSeries =
  process.env.MOESIF_TEMPLATE_WORKSPACE_ID_TIME_SERIES;

const jsonParser = bodyParser.json();

if (!moesifApplicationId) {
  console.error(
    "No MOESIF_APPLICATION_ID found. Please create an .env file with MOESIF_APPLICATION_ID."
  );
  throw new Error("No MOESIF_APPLICATION_ID found.");
}

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
if (!provisioningService) {
  console.error("No provisioning service found!");
  throw new Error("No provisioning service found!");
}
const customBillingProvider = new BillingProvider();

const moesifMiddleware = moesif({
  applicationId: moesifApplicationId,
  identifyUser: function (req: any, _res: any) {
    return getUnifiedCustomerIdCached(req?.user);
  },
});

app.use(moesifMiddleware, cors());

// Extend Express Request type to include user
interface UserRequest extends Request {
  user?: any;
}

app.post(
  "/create-stripe-checkout-session",
  authMiddleware,
  async (req: UserRequest, res: Response) => {
    const priceId = req.query?.price_id as string;
    const email = req.user?.email;
    const quantity = req.query?.quantity as string | undefined;

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

app.get("/plans", jsonParser, async (_req: Request, res: Response) => {
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
  authMiddleware,
  jsonParser,
  async (req: UserRequest, res: Response) => {
    const sanitizedEmail = (req.query.email as string).replace(/\n|\r/g, "");
    console.log("query email " + sanitizedEmail);
    console.log("verified email from claims " + req.user.email);
    const email = req.user?.email;

    let moesifUserId: string | undefined;
    try {
      moesifUserId = await getUnifiedCustomerId(req.user, email);
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
    } catch (err: any) {
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

app.post(
  "/register/stripe/:checkout_session_id",
  authMiddleware,
  function (req: UserRequest, res: Response) {
    const checkout_session_id = req.params.checkout_session_id;

    verifyStripeSession(checkout_session_id)
      .then(async (result: any) => {
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
              syncToMoesif({
                companyId: stripe_subscription_id,
                // TODO: check if we need `subscriptionId`
                // subscriptionId: stripe_subscription_id,
                userId: stripe_customer_id,
                email: email,
              });
            } else {
              console.log("updating company and user with V2");
              syncToMoesif({
                companyId: stripe_customer_id,
                // TODO: check if we need `subscriptionId`
                // subscriptionId: stripe_subscription_id,
                userId: stripe_customer_id,
                email: email,
              });
            }
          } catch (error) {
            console.error("Error updating user/company/sub:", error);
          }

          const user = await provisioningService.provisionUser(
            stripe_customer_id,
            email,
            stripe_subscription_id
          );
          console.log("provisioned user:", JSON.stringify(user));
        }
        console.log(JSON.stringify(stripeCheckOutSessionInfo));
        res.status(201).json(stripeCheckOutSessionInfo);
      })
      .catch((err: any) => {
        console.error("Error registering user", err);
        res.status(500).json({
          message: "Failed to provision user. " + err.toString(),
        });
      });
  }
);

app.post(
  "/register/custom",
  authMiddleware,
  jsonParser,
  async function (req: UserRequest, res: Response) {
    const customerId = await getUnifiedCustomerId(req.user);
    if (!customerId) {
      console.error(
        `No customerId found for current user ${JSON.stringify(req.user)}`
      );
      throw new Error("No customerId found");
    }
    const email = req.user?.email;
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
        metadata: {},
      });

      const user = await provisioningService.provisionUser(
        customerId,
        email,
        subscription.id
      );
      res.status(201).json({ status: "provisioned" });
    } catch (err: any) {
      console.error("Error registering user", err);
      res.status(500).json({
        message: "Failed to provision user. " + err.toString(),
      });
    }
  }
);

app.get(
  "/stripe/customer",
  authMiddleware,
  function (req: UserRequest, res: Response) {
    const email = req.user?.email;

    getStripeCustomer(email)
      .then((result: any) => {
        if (result.data && result.data[0]) {
          res.status(200).json(result.data[0]);
        } else {
          res.status(404).json("stripe customer not found");
        }
      })
      .catch((err: any) => {
        console.error("Error getting customer info from stripe", err);
        res.status(500).json({
          message: "Failed to retrieve customer info from stripe",
        });
      });
  }
);

app.post(
  "/create-key",
  authMiddleware,
  jsonParser,
  async function (req: UserRequest, res: Response) {
    try {
      const email = req.user?.email;

      const customerId = await getUnifiedCustomerId(req.user, email);
      if (!customerId) {
        throw new Error(
          `Customer Id unknown. Ensure you're subscribed to a plan. If you just subscribed, try again.`
        );
      }

      const apiKey = await provisioningService.createApiKey(customerId, email);
      res.status(200).send({ apikey: apiKey });
    } catch (error) {
      console.error("Error creating key:", error);
      res.status(500).json({ message: "Failed to create key" });
    }
  }
);

app.get(
  "/embed-charts(/:authUserId)",
  authMiddleware,
  async function (req: UserRequest, res: Response) {
    const authUserId = req.user?.sub;
    const email = req.user?.email;

    try {
      const customerId = await getUnifiedCustomerId(req.user, email);
      if (!customerId) {
        console.error("Customer Id not found when fetching for " + email);
      }

      const embedInfoArray = await Promise.all(
        [templateWorkspaceIdLiveEvent, templateWorkspaceIdTimeSeries].map(
          (workspaceId) =>
            getInfoForEmbeddedWorkspaces({
              workspaceId: workspaceId!,
              userId: customerId!,
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
