import { Link } from "react-router";
import { Button } from "~/components/ui/button";
import type {
  Plan,
  PlanMetadata,
  Price,
} from "~/features/dashboard/moesif.server";

const BILLING_INTERVAL_LABELS: Record<string, string> = {
  D: "day",
  W: "week",
  M: "month",
  Y: "year",
};

function needQuantity(price: Price): boolean {
  // Stripe price object: usage_type === 'metered' means do NOT include quantity
  // For other pricing models, quantity is required
  // If price has price meter or usage_aggregator, quantity is not needed
  if (price.usage_type === "metered") return false;
  if (price.price_meter) return false;
  if (price.usage_aggregator) return false;
  // Otherwise, quantity is needed
  return true;
}

function formatPrice(price: Price): string {
  const value = Number(price.price_in_decimal);
  const currency = (price.currency || "usd").toUpperCase();

  if (!Number.isFinite(value)) {
    return currency;
  }

  const formatter = new Intl.NumberFormat("en-US", {
    style: "currency",
    currency,
    minimumFractionDigits: 0,
    maximumFractionDigits: 2,
  });

  const periodUnits = price.period_units ?? "";
  const unitLabel = BILLING_INTERVAL_LABELS[periodUnits] || periodUnits;
  const periodValue = price.period ?? 1;
  const baseLabel = unitLabel || "period";
  const pluralizedLabel =
    periodValue > 1 && !baseLabel.endsWith("s") ? `${baseLabel}s` : baseLabel;
  const interval =
    periodValue > 1 ? `${periodValue} ${pluralizedLabel}` : baseLabel;

  return `${formatter.format(value)} / ${interval}`;
}

function extractFeatures(plan: Plan): string[] {
  const metadata = (
    plan.metadata && typeof plan.metadata === "object" ? plan.metadata : {}
  ) as PlanMetadata;
  const rawFeatures =
    metadata.features ?? metadata.feature_bullets ?? metadata.features_list;

  if (Array.isArray(rawFeatures)) {
    return rawFeatures
      .map((feature) =>
        typeof feature === "string"
          ? feature
          : typeof feature === "number"
          ? feature.toString()
          : null
      )
      .filter((feature): feature is string => Boolean(feature));
  }

  return [];
}

interface PlanCardGridProps {
  plans: Plan[];
  ctaLabel?: string;
  disabled?: boolean;
}

export function PlanCardGrid({
  plans,
  ctaLabel = "Get Started",
  disabled = true,
}: PlanCardGridProps) {
  return (
    <div className="grid gap-8 sm:grid-cols-2 xl:grid-cols-3">
      {plans.map((plan) => {
        const primaryPrice = plan.prices[0];
        const additionalPrices = plan.prices.slice(1);
        const features = extractFeatures(plan);
        let ctaHref = `/checkout?plan_id=${plan.id}&price_id=${primaryPrice.id}`;
        if (needQuantity(primaryPrice)) {
          ctaHref += `&quantity=1`;
        }

        return (
          <article
            key={plan.id}
            className="flex flex-col justify-between rounded-2xl border bg-background/60 p-6 shadow-sm"
          >
            <div>
              <h3 className="text-xl font-semibold">{plan.name}</h3>
              {plan.description && (
                <p className="mt-2 text-sm text-muted-foreground">
                  {plan.description}
                </p>
              )}

              {primaryPrice && (
                <div className="mt-6">
                  <span className="text-3xl font-bold">
                    {formatPrice(primaryPrice)}
                  </span>
                </div>
              )}

              {additionalPrices.length > 0 && (
                <div className="mt-4 space-y-1 text-sm text-muted-foreground">
                  {additionalPrices.map((price) => (
                    <div key={price.id}>{formatPrice(price)}</div>
                  ))}
                </div>
              )}

              {features.length > 0 && (
                <ul className="mt-6 space-y-2 text-sm text-muted-foreground">
                  {features.map((feature) => (
                    <li key={feature} className="flex gap-2">
                      <span aria-hidden="true">•</span>
                      <span>{feature}</span>
                    </li>
                  ))}
                </ul>
              )}
            </div>

            <div className="mt-8">
              {disabled ? (
                <Button className="w-full" disabled>
                  {ctaLabel}
                </Button>
              ) : (
                <Link to={ctaHref}>
                  <Button className="w-full">{ctaLabel}</Button>
                </Link>
              )}
            </div>
          </article>
        );
      })}
    </div>
  );
}
