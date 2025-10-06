import express, { Request, Response, NextFunction } from "express";
import path from "path";
import dotenv from "dotenv";
import bodyParser from "body-parser";
import moesif from "moesif-nodejs";
import cors from "cors";
import { traceMiddleware } from "./middlewares/trace.middleware";
import pinoHttpMiddleware from "./middlewares/pinoHttp.middleware";

// Load environment variables
dotenv.config({ path: [".env", ".env.template"] });

import "./common/instrumentation";

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
import { getLogger } from "./common/logger.context";
import logger from "./common/logger";
import { withSpan } from "./decorators/span";

const app = express();
app.use(pinoHttpMiddleware);
// Add traceability middleware
app.use(traceMiddleware);
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
  logger.fatal(
    "No MOESIF_APPLICATION_ID found. Please create an .env file with MOESIF_APPLICATION_ID."
  );
  throw new Error("No MOESIF_APPLICATION_ID found.");
}

if (!moesifManagementToken) {
  logger.fatal(
    "No MOESIF_MANAGEMENT_TOKEN found. Please create an .env file with MOESIF_MANAGEMENT_TOKEN & MOESIF_TEMPLATE_WORKSPACE_ID."
  );
}

if (!templateWorkspaceIdLiveEvent) {
  logger.fatal(
    "No MOESIF_TEMPLATE_WORKSPACE_ID found. Please create an .env file with MOESIF_MANAGEMENT_TOKEN & MOESIF_TEMPLATE_WORKSPACE_ID."
  );
}

const provisioningService = getApimProvisioningPlugin();
if (!provisioningService) {
  logger.fatal("No provisioning service found!");
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
  withSpan(
    "createStripeCheckoutSession",
    async (req: UserRequest, res: Response) => {
      const priceId = req.query?.price_id as string;
      const email = req.user?.email;
      const quantity = req.query?.quantity as string | undefined;

      const logger = getLogger();

      logger.info(
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
        logger.info("got session back from stripe session");
        logger.info(JSON.stringify(session));

        res.send({ clientSecret: session.client_secret });
      } catch (err) {
        logger.error("Failed to create stripe checkout session", err);
        res.status(400).json({ message: "Error creating check out session" });
      }
    }
  )
);

app.get(
  "/plans",
  jsonParser,
  withSpan("getPlans", async (_req: Request, res: Response) => {
    const logger = getLogger();
    try {
      const result = await getPlansFromMoesif();
      logger.trace(`result: ${JSON.stringify(result)}`);
      res.status(200).json(result);
    } catch (err) {
      logger.error("Error getting plans from Moesif", err);
      res.status(500).json({ message: "Error getting plans from Moesif" });
    }
  })
);

app.get(
  "/subscriptions",
  authMiddleware,
  jsonParser,
  withSpan("getSubscriptions", async (req: UserRequest, res: Response) => {
    const logger = getLogger();

    const sanitizedEmail = (req.query.email as string).replace(/\n|\r/g, "");
    logger.info("query email " + sanitizedEmail);
    logger.info("verified email from claims " + req.user.email);
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

      logger.info(
        "got subscriptions from moesif " + JSON.stringify(subscriptions)
      );
      res.status(200).json(subscriptions);
    } catch (err: any) {
      logger.error(
        "Error getting subscription from moesif for " +
          email +
          " " +
          moesifUserId,
        err
      );
      res.status(404).json({ message: err.toString() });
    }
  })
);

app.post(
  "/register/stripe/:checkout_session_id",
  authMiddleware,
  withSpan(
    {
      name: "registerStripeCheckoutSession",
      extractAttributesFromArgs: (args) => ({
        checkout_session_id: args[0].params.checkout_session_id,
      }),
    },
    async function (req: UserRequest, res: Response) {
      const logger = getLogger();

      const checkout_session_id = req.params.checkout_session_id;
      logger.debug(`checkout_session_id = ${checkout_session_id}`);

      try {
        const session = await verifyStripeSession(checkout_session_id);
        const stripeCheckOutSessionInfo = session;
        logger.info("in stripe register");

        if (session.customer && session.subscription) {
          logger.info("customer and subscription present");
          const email = session.customer_details?.email;
          if (!email) {
            logger.error(
              `email is ${typeof email === "string" ? "empty" : email}`
            );
            throw new Error(`email not found`);
          }
          const stripe_customer_id =
            typeof session.customer === "object"
              ? session.customer.id
              : session.customer;
          const stripe_subscription_id =
            typeof session.subscription === "object"
              ? session.subscription.id
              : session.subscription;
          try {
            if (
              process.env.MOESIF_MONETIZATION_VERSION &&
              process.env.MOESIF_MONETIZATION_VERSION.toUpperCase() === "V1"
            ) {
              logger.info("updating company and user with V1");
              syncToMoesif({
                companyId: stripe_subscription_id,
                // TODO: check if we need `subscriptionId`
                // subscriptionId: stripe_subscription_id,
                userId: stripe_customer_id,
                email: email,
              });
            } else {
              logger.info("updating company and user with V2");
              syncToMoesif({
                companyId: stripe_customer_id,
                // TODO: check if we need `subscriptionId`
                // subscriptionId: stripe_subscription_id,
                userId: stripe_customer_id,
                email,
              });
            }
          } catch (error) {
            logger.error("Error updating user/company/sub:", error);
          }

          const user = await provisioningService.provisionUser(
            stripe_customer_id,
            email,
            stripe_subscription_id
          );
          logger.info("provisioned user:", JSON.stringify(user));
        }
        logger.info(JSON.stringify(stripeCheckOutSessionInfo));
        res.status(201).json(stripeCheckOutSessionInfo);
      } catch (err) {
        logger.error("Error registering user", err);
        res.status(500).json({
          message: `Failed to provision user. ${err}`,
        });
      }
    }
  )
);

// This endpoint is still active and instrumented for tracing.
// If the custom billing provider is fully deprecated, consider removing this endpoint in the future.
app.post(
  "/register/custom",
  authMiddleware,
  jsonParser,
  withSpan("registerCustom", async function (req: UserRequest, res: Response) {
    const logger = getLogger();

    const customerId = await getUnifiedCustomerId(req.user);
    if (!customerId) {
      logger.error(
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

      logger.info("custom subscription created", subscription);

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
      logger.error("Error registering user", err);
      res.status(500).json({
        message: "Failed to provision user. " + err.toString(),
      });
    }
  })
);

app.get(
  "/stripe/customer",
  authMiddleware,
  withSpan("getStripeCustomer", function (req: UserRequest, res: Response) {
    const logger = getLogger();
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
        logger.error("Error getting customer info from stripe", err);
        res.status(500).json({
          message: "Failed to retrieve customer info from stripe",
        });
      });
  })
);

app.post(
  "/create-key",
  authMiddleware,
  jsonParser,
  withSpan(
    {
      name: "createKey",
      extractAttributesFromArgs: (args) => ({
        email: args[0].user?.email,
      }),
    },
    async function (req: UserRequest, res: Response) {
      const logger = getLogger();
      try {
        const email = req.user?.email;

        const customerId = await getUnifiedCustomerId(req.user, email);
        if (!customerId) {
          throw new Error(
            `Customer Id unknown. Ensure you're subscribed to a plan. If you just subscribed, try again.`
          );
        }

        const apiKey = await provisioningService.createApiKey(
          customerId,
          email
        );
        res.status(200).send({ apikey: apiKey });
      } catch (error) {
        logger.error("Error creating key:", error);
        res.status(500).json({ message: "Failed to create key" });
      }
    }
  )
);

app.get(
  "/embed-charts(/:authUserId)",
  authMiddleware,
  withSpan("getEmbedCharts", async function (req: UserRequest, res: Response) {
    const logger = getLogger();
    const authUserId = req.user?.sub;
    const email = req.user?.email;

    try {
      const customerId = await getUnifiedCustomerId(req.user, email);
      if (!customerId) {
        logger.error("Customer Id not found when fetching for " + email);
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
      logger.error("Error generating embedded templates:", err);
      res.status(500).json({ message: "Failed to retrieve embedded template" });
    }
  })
);

app.listen(port, () => {
  logger.info(
    `Zama Developer Portal Backend is listening at http://localhost:${port}`
  );
});
