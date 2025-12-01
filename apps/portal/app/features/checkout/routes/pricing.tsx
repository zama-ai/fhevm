import type { Route } from "./+types/pricing";
import type { Plan } from "~/features/dashboard/moesif.server";
import { PlanCardGrid } from "~/components/pricing/plan-grid";
import { getUserFromSession } from "~/features/auth/session.server";
import {
  getCustomerSubscriptions,
  getStripeCustomer,
} from "~/features/checkout/stripe.server";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Pricing | Zama Console" },
    {
      name: "description",
      content:
        "Choose the right Zama Console subscription plan to build fully homomorphic encryption applications.",
    },
  ];
}

async function getUserPlans(request: Request): Promise<string[]> {
  const user = await getUserFromSession(request);
  if (user) {
    let customer = await getStripeCustomer(user.email);
    if (customer) {
      console.log("Stripe customer found: ", customer.id);
      const subscriptions = await getCustomerSubscriptions(customer.id);
      console.log(`Found ${subscriptions.length} subscriptions`);
      return subscriptions
        .map((s) => s.items.data[0].plan.product)
        .map((p) => (p ? (typeof p === "string" ? p : p.id) : null))
        .filter((p): p is string => p !== null);
    }
  }
  return [];
}

async function fetchPlans(): Promise<{ plans: Plan[]; error?: string }> {
  try {
    const { getPlans } = await import("~/features/dashboard/moesif.server");
    const plans = await getPlans();
    return { plans };
  } catch (error) {
    return {
      plans: [],
      error:
        error instanceof Error
          ? error.message
          : "Failed to load subscription plans. Please try again later.",
    };
  }
}

export async function loader({ request }: Route.LoaderArgs): Promise<{
  plans: Plan[];
  planIds: string[];
  error?: string;
}> {
  const [{ plans, error }, planIds] = await Promise.all([
    fetchPlans(),
    getUserPlans(request),
  ]);

  return { plans, planIds, error };
}

export default function Pricing({ loaderData }: Route.ComponentProps) {
  const { plans, planIds, error } = loaderData;

  return (
    <div className="container mx-auto max-w-6xl py-16">
      <div className="text-center mb-12">
        <h1 className="text-4xl font-bold tracking-tight sm:text-5xl">
          Subscription Plans
        </h1>
        {planIds.length > 0 ? (
          <p className="mt-4 text-lg text-muted-foreground">
            You are currently subscribed to:{" "}
            {planIds
              .map((id) => ({ id, name: plans.find((p) => p.id === id)?.name }))
              .map(({ id, name }, idx, ids) => (
                <>
                  <span key={id} className="font-medium text-primary">
                    {name || id}
                  </span>
                  {idx < ids.length - 1 && ", "}
                </>
              ))}
          </p>
        ) : (
          <p className="mt-4 text-lg text-muted-foreground">
            Pick the plan that matches your stage of FHE application
            development.
          </p>
        )}
      </div>

      {error && (
        <div className="mx-auto max-w-2xl mb-12 rounded-md border border-destructive/50 bg-destructive/10 p-4 text-destructive">
          {error}
        </div>
      )}

      {plans.length === 0 && !error ? (
        <div className="text-center text-muted-foreground">
          Subscription plans will appear here once they are configured in
          Moesif.
        </div>
      ) : (
        <PlanCardGrid plans={plans} disabled={planIds.length > 0} />
      )}
    </div>
  );
}
