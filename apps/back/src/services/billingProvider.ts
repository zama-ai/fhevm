import crypto from "crypto";
import { Request } from "express";

function getOneMonthFromNowISO(): string {
  const now = new Date(); // Get the current date and time
  now.setMonth(now.getMonth() + 1); // Add one calendar month
  return now.toISOString(); // Convert to ISO 8601 format
}

export interface Subscription {
  id: string;
  plan_id?: string;
  price_id?: string;
  current_period_start: string;
  current_period_end: string;
  [key: string]: any;
}

export interface VerifyPurchaseData {
  plan_id?: string;
  price_id?: string;
  [key: string]: any;
}

export class BillingProvider {
  verifyPurchaseAndCreateSubscription(
    req: Request,
    data: VerifyPurchaseData
  ): { subscription: Subscription } {
    // use the request info to verify the purchase after checkout
    // but generally your billing provider should verify the subscription
    // and return the subscription object.
    // below is a fake subscription generated on the fly.

    const subscription: Subscription = {
      id: crypto.randomUUID(),
      plan_id: data?.plan_id,
      price_id: data?.price_id,
      ...data,
      current_period_start: new Date().toISOString(),
      current_period_end: getOneMonthFromNowISO(),
    };

    return {
      subscription,
    };
  }
}
