import config from "../config";
import { useQuery } from "@tanstack/react-query";

export type Plan = {
  provider: "stripe";
  id: string;
  name: string;
  description: string;
  status: "active" | "inactive";
  prices: Price[];
  unit?: string;
};

export type Price = {
  provider: "stripe";
  id: string;
  name?: string;
  status: "active" | "inactive";
  currency: string;
  metadata: Record<string, any>;
  plan_id: string;
  period: number;
  period_units: "M" | "y";
  price_in_decimal: string | number;
  pricing_model: "flat" | "per_unit";
  usage_type?: "metered";
  price_meter?: null | object;
  usage_aggregator?: null | object;
  tiers?: Tier[];
};

export type Tier = {
  up_to: number | string;
  unit_price_in_decimal: string;
  flat_price_in_decimal: string;
};

export default function usePlans() {
  const { data, error, isLoading } = useQuery({
    queryKey: ["plans"],
    queryFn: async () => {
      const response = await fetch(`${config.devPortalApiServer}/plans`);
      if (!response.ok) {
        console.error(
          `Failed to fetch plans. Status: ${response.status} ${response.statusText}`
        );
        throw new Error(
          `Failed to fetch plans: ${response.status} ${response.statusText}`
        );
      }
      const result: { hits: Plan[] } = await response.json();
      return (result?.hits || []).filter(function (item) {
        return item.status === "active";
      });
    },
  });

  return {
    plans: data,
    plansLoading: isLoading,
    plansError: error,
  };
}
