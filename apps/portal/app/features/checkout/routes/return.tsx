import { Link, useNavigate, type LoaderFunctionArgs } from "react-router";
import { useEffect } from "react";
import { provisionUser } from "~/features/api-keys/kong.server";
import { verifyStripeSession } from "~/features/checkout/stripe.server";
import type { Route } from "./+types/return";
import { CheckCircle2, AlertTriangle, Loader2 } from "lucide-react";
import { Button } from "~/components/ui/button";
import { cn } from "~/lib/utils";

export async function loader({ request }: LoaderFunctionArgs) {
  const url = new URL(request.url);
  const sessionId = url.searchParams.get("session_id");
  const priceId = url.searchParams.get("price_id");

  if (!sessionId) {
    throw new Error("Missing session_id in return URL");
  }

  const session = await verifyStripeSession(sessionId);
  if (session.customer && session.subscription) {
    const email = session.customer_details?.email;
    if (!email) {
      console.warn("No email found in Stripe session customer details");
      throw new Error("No email found in Stripe session customer details");
    }
    const stripe_customer_id =
      typeof session.customer === "object"
        ? session.customer.id
        : session.customer;
    const stripe_subscription_id =
      typeof session.subscription === "object"
        ? session.subscription.id
        : session.subscription;

    console.log(
      `Stripe session verified for customer ${email}, subscription ${stripe_subscription_id}`
    );

    const { syncToMoesif } = await import("~/features/dashboard/moesif.server");
    if (import.meta.env.MOESIF_MONETIZATION_VERSION?.toUpperCase() === "V1") {
      console.log("updating company and user with V1");
      // in v1, companyId and subscription id has one to one mapping.
      syncToMoesif({
        companyId: stripe_subscription_id,
        subscriptionId: stripe_subscription_id,
        userId: stripe_customer_id,
        email: email,
      });
    } else {
      console.log("Updating company and user with V2");
      // assume you have one user per subscription
      // but if you have multiple users per each subscription
      // please check out https://www.moesif.com/docs/getting-started/overview/
      // for the different entities how they are related to each other.
      await syncToMoesif({
        companyId: stripe_customer_id,
        subscriptionId: stripe_subscription_id,
        userId: stripe_customer_id,
        email: email,
      });
    }

    try {
      // Provision new user to access to API
      await provisionUser({
        customerId: stripe_customer_id,
        email,
      });
    } catch (error) {
      console.error("Error provisioning user in Kong:", error);
    }
  }

  return { session, sessionId, priceId };
}

export default function Return({ loaderData }: Route.ComponentProps) {
  const { session, sessionId, priceId } = loaderData;
  const navigate = useNavigate();
  const status = session.status ?? "unknown";

  useEffect(() => {
    if (status === "open") {
      navigate(priceId ? `/checkout?price_id=${priceId}` : "/pricing", {
        replace: true,
      });
    }
  }, [status, priceId, navigate]);

  if (status === "open") {
    return (
      <section className="mx-auto flex min-h-[60vh] w-full max-w-2xl flex-col items-center justify-center px-6 py-16">
        <div className="w-full rounded-3xl border border-primary/30 bg-primary/5 p-10 text-center shadow-lg shadow-primary/10 backdrop-blur">
          <Loader2 className="mx-auto mb-6 size-12 animate-spin text-primary" />
          <h1 className="text-3xl font-semibold tracking-tight text-foreground">
            Finishing up your checkout
          </h1>
          <p className="mt-3 text-base text-muted-foreground">
            We&apos;re confirming your payment details. You&apos;ll be
            redirected in a moment.
          </p>
        </div>
      </section>
    );
  }

  const isSuccess = status === "complete";
  const Icon = isSuccess ? CheckCircle2 : AlertTriangle;
  const toneCard = cn(
    "w-full rounded-3xl border p-10 shadow-lg backdrop-blur transition-shadow",
    isSuccess
      ? "border-primary/40 bg-primary/10 shadow-primary/15"
      : "border-destructive/30 bg-destructive/10 shadow-destructive/20"
  );
  const toneIcon = cn(
    "mb-6 size-12",
    isSuccess ? "text-primary" : "text-destructive"
  );
  const heading = isSuccess
    ? "You're all set!"
    : "We couldn't confirm your payment";
  const bodyCopy = isSuccess
    ? "Thank you for your purchase. Your subscription is now active and you can start building with the portal right away."
    : "We weren't able to confirm this transaction. You can try again or reach out if you need a hand.";

  return (
    <section className="mx-auto flex min-h-[60vh] w-full max-w-2xl flex-col items-center justify-center px-6 py-16">
      <div className={toneCard}>
        <Icon className={toneIcon} aria-hidden="true" />
        <h1 className="text-3xl font-semibold tracking-tight text-foreground">
          {heading}
        </h1>
        <p className="mt-3 text-base text-muted-foreground">{bodyCopy}</p>

        <dl className="mt-8 space-y-3 text-sm text-muted-foreground">
          <div className="flex items-center justify-between">
            <dt className="font-medium text-foreground">Status</dt>
            <dd className="capitalize text-foreground/80">
              {status.replace(/_/g, " ")}
            </dd>
          </div>
          {session.payment_status ? (
            <div className="flex items-center justify-between">
              <dt className="font-medium text-foreground">Payment</dt>
              <dd className="capitalize text-foreground/80">
                {session.payment_status.replace(/_/g, " ")}
              </dd>
            </div>
          ) : null}
          {sessionId ? (
            <div className="flex items-center justify-between">
              <dt className="font-medium text-foreground">Session ID</dt>
              <dd className="font-mono text-xs text-muted-foreground">
                {sessionId}
              </dd>
            </div>
          ) : null}
        </dl>

        <div className="mt-10 flex flex-wrap gap-3">
          {isSuccess ? (
            <>
              <Button asChild className="min-w-[160px] flex-1">
                <Link to="/dashboard">Go to dashboard</Link>
              </Button>
              <Button variant="ghost" asChild className="min-w-[160px] flex-1">
                <Link to="/api-keys">Manage API keys</Link>
              </Button>
            </>
          ) : (
            <>
              <Button
                variant="outline"
                asChild
                className="min-w-[160px] flex-1"
              >
                <Link to="/pricing">Try checkout again</Link>
              </Button>
              <Button variant="ghost" asChild className="min-w-[160px] flex-1">
                <a
                  href="https://community.zama.ai/"
                  target="_blank"
                  rel="noreferrer"
                >
                  Visit support forum
                </a>
              </Button>
            </>
          )}
        </div>
      </div>
    </section>
  );
}
