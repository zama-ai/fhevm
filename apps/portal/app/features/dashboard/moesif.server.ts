import moesif from "moesif-nodejs";

const moesifApiEndPoint = "https://api.moesif.com";

const MOESIF_APPLICATION_ID = process.env.MOESIF_APPLICATION_ID;
const MOESIF_MANAGEMENT_TOKEN = process.env.MOESIF_MANAGEMENT_TOKEN;
const MOESIF_TEMPLATE_WORKSPACE_ID_LIVE_EVENT_LOG =
  process.env.MOESIF_TEMPLATE_WORKSPACE_ID_LIVE_EVENT_LOG;
const MOESIF_TEMPLATE_WORKSPACE_ID_TIME_SERIES =
  process.env.MOESIF_TEMPLATE_WORKSPACE_ID_TIME_SERIES;

const APP_PAYMENT_PROVIDER = process.env.APP_PAYMENT_PROVIDER || "stripe";
const RAW_CACHE_SECONDS = Number(
  process.env.MOESIF_PLANS_CACHE_SECONDS ?? "300"
);
const DEFAULT_CACHE_SECONDS = 300;
const PLANS_CACHE_TTL_MS =
  Number.isFinite(RAW_CACHE_SECONDS) && RAW_CACHE_SECONDS >= 0
    ? RAW_CACHE_SECONDS * 1000
    : DEFAULT_CACHE_SECONDS * 1000;

if (!MOESIF_APPLICATION_ID || !MOESIF_MANAGEMENT_TOKEN) {
  throw new Error("Missing Moesif configuration in environment variables");
}

const moesifClient = moesif({
  applicationId: MOESIF_APPLICATION_ID,
});

type PlanStatus = "active" | "inactive" | "archived" | "draft" | string;
type PriceStatus = "active" | "inactive" | "archived" | "draft" | string;
type PricingModel =
  | "flat"
  | "per_unit"
  | "package"
  | "tiered"
  | "volume"
  | string;
type TaxBehavior = "inclusive" | "exclusive" | "unspecified" | string;
type PeriodUnits = "S" | "M" | "W" | "D" | "Y" | string;

export interface TransformQuantity {
  divide_by?: number;
  round?: string;
}

export interface MoesifPriceTier {
  flat_currency_prices?: Record<string, string> | null;
  up_to?: number | null;
  flat_price_in_decimal?: number | null;
  unit_price_in_decimal?: number | null;
  unit_currency_prices?: Record<string, string> | null;
}

export interface PriceMetadata extends Record<string, unknown> {
  checkout_url?: unknown;
  strapi_checkout_url?: unknown;
}

export interface Price {
  id: string;
  provider: string;
  status: PriceStatus;
  currency: string;
  pricing_model?: PricingModel;
  tax_behavior?: TaxBehavior;
  period?: number | null;
  period_units?: PeriodUnits;
  plan_id?: string;
  name?: string;
  price_in_decimal?: number | string;
  metadata?: PriceMetadata | null;
  created_at?: string;
  unit?: string;
  deferred_revenue_accounting_code?: string | null;
  recognized_revenue_accounting_code?: string | null;
  revenue_recognition_rule?: string | null;
  usage_aggregator?: string | null;
  usage_type?: "licensed" | "metered" | string;
  price_meter?: unknown;
  transform_quantity?: TransformQuantity | null;
  currency_prices?: Record<string, string> | null;
  tiers?: MoesifPriceTier[] | null;
  price?: number | null;
}

export interface PlanMetadata extends Record<string, unknown> {
  features?: unknown;
  feature_bullets?: unknown;
  features_list?: unknown;
  checkout_url?: unknown;
}

export interface Plan {
  id: string;
  provider: string;
  status: PlanStatus;
  name: string;
  description?: string;
  billing_type?: string;
  external_plan_id?: string;
  product_id?: string;
  billing_period?: string;
  reporting_period?: string;
  unit?: string;
  created_at?: string;
  updated_at?: string;
  metadata?: PlanMetadata | null;
  prices: Price[];
}

type MoesifPlansResponse = {
  hits?: Plan[];
  plans?: Plan[];
};

type PlansCacheEntry = {
  value: Plan[];
  expiresAt: number;
};

let plansCache: PlansCacheEntry | undefined;
let inflightPlansRequest: Promise<Plan[]> | undefined;

async function fetchPlansFromMoesif(): Promise<Plan[]> {
  const response = await fetch(
    `${moesifApiEndPoint}/v1/~/billing/catalog/plans?includes=prices&provider=${APP_PAYMENT_PROVIDER}`,
    {
      method: "GET",
      headers: {
        Authorization: `Bearer ${MOESIF_MANAGEMENT_TOKEN}`,
      },
    }
  );

  if (!response.ok) {
    const errorBody = await response.text();
    throw new Error(
      `Failed to fetch Moesif plans: ${response.status} ${errorBody}`
    );
  }

  const data = (await response.json()) as MoesifPlansResponse;
  const rawPlans = Array.isArray(data.hits)
    ? data.hits
    : Array.isArray(data.plans)
    ? data.plans
    : [];

  return rawPlans
    .filter((plan) => plan.status === "active")
    .map((plan) => ({
      ...plan,
      metadata: (plan.metadata ?? {}) as PlanMetadata,
      prices: Array.isArray(plan.prices)
        ? plan.prices
            .filter((price) => price.status === "active")
            .map((price) => ({
              ...price,
              metadata: (price.metadata ?? {}) as PriceMetadata,
            }))
        : [],
    }))
    .filter((plan) => plan.prices.length > 0);
}

async function refreshPlansCache(): Promise<Plan[]> {
  const plans = await fetchPlansFromMoesif();
  if (PLANS_CACHE_TTL_MS > 0) {
    plansCache = {
      value: plans,
      expiresAt: Date.now() + PLANS_CACHE_TTL_MS,
    };
  } else {
    plansCache = undefined;
  }
  return plans;
}

export function clearPlansCache() {
  plansCache = undefined;
}

export async function getPlans({
  forceRefresh = false,
}: { forceRefresh?: boolean } = {}): Promise<Plan[]> {
  const now = Date.now();

  if (!forceRefresh && plansCache && plansCache.expiresAt > now) {
    return plansCache.value;
  }

  if (PLANS_CACHE_TTL_MS === 0 || forceRefresh) {
    return refreshPlansCache();
  }

  if (!inflightPlansRequest) {
    inflightPlansRequest = refreshPlansCache().finally(() => {
      inflightPlansRequest = undefined;
    });
  }

  return inflightPlansRequest;
}

export async function syncToMoesif({
  companyId,
  userId,
  subscriptionId,
  email,
}: {
  companyId?: string;
  userId?: string;
  subscriptionId?: string;
  email?: string;
}) {
  if (companyId) {
    console.log("Updating Moesif company:", companyId);
    moesifClient.updateCompany({
      companyId,
      metadata: {
        // feel free to add additonal profile data here.
      },
    });
  }
  if (userId) {
    console.log("Updating Moesif user:", userId);
    moesifClient.updateUser({
      userId,
      companyId,
      metadata: {
        // feel free to add additonal profile data here.
        email,
        subscriptionId,
      },
    });
  }
}

export async function getInfoForEmbeddedWorkspaces(userId: string) {
  const embedInfoArray = await Promise.all(
    [
      MOESIF_TEMPLATE_WORKSPACE_ID_LIVE_EVENT_LOG!,
      MOESIF_TEMPLATE_WORKSPACE_ID_TIME_SERIES!,
    ].map((workspaceId) => getInfoForEmbeddedWorkspace({ userId, workspaceId }))
  );
  return embedInfoArray;
}

async function getInfoForEmbeddedWorkspace({
  userId,
  workspaceId,
}: {
  userId: string;
  workspaceId: string;
}): Promise<{ token: string; _id: string; url: string }> {
  const tomorrow = new Date();
  tomorrow.setDate(tomorrow.getDate() + 7);
  const expiration = tomorrow.toISOString();

  const moesif_url_live_event = `${moesifApiEndPoint}/v1/portal/~/workspaces/${workspaceId}/access_token?expiration=${expiration}`;

  const response = await fetch(moesif_url_live_event, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${MOESIF_MANAGEMENT_TOKEN}`,
    },
    body: JSON.stringify({
      template: {
        values: {
          user_id: userId,
        },
      },
    }),
  });

  if (!response.ok) {
    const errorBody = await response.text();
    throw new Error(
      `Failed to fetch Moesif embedded workspace info: ${response.status} ${errorBody}`
    );
  }

  const data = await response.json();
  return data;
}
