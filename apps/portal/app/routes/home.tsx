import type { Route } from "./+types/home";
import { Link } from "react-router";
import { Button } from "~/components/ui/button";
import { PlanCardGrid } from "~/components/pricing/plan-grid";
import type { Plan } from "~/features/dashboard/moesif.server";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Zama Console - Secure FHE Development Platform" },
    {
      name: "description",
      content:
        "Build privacy-preserving applications with Fully Homomorphic Encryption",
    },
  ];
}

export async function loader({}: Route.LoaderArgs): Promise<{
  plans: Plan[];
  error?: string;
}> {
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

export default function Home({ loaderData }: Route.ComponentProps) {
  const { plans, error } = loaderData;

  return (
    <div className="flex flex-col">
      {/* Hero Section */}
      <section className="container mx-auto flex flex-col items-center justify-center gap-6 pb-8 pt-6 md:py-24">
        <div className="flex max-w-[980px] flex-col items-center gap-4 text-center">
          <h1 className="text-4xl font-extrabold leading-tight tracking-tighter md:text-6xl lg:text-7xl">
            Build Private Applications
            <br className="hidden sm:inline" /> with Fully Homomorphic
            Encryption
          </h1>
          <p className="max-w-[750px] text-lg text-muted-foreground sm:text-xl">
            Zama Console provides everything you need to develop, test, and
            deploy privacy-preserving applications using cutting-edge FHE
            technology.
          </p>
        </div>

        <div className="flex gap-4">
          <Link to="/auth/signup">
            <Button size="lg" className="h-11">
              Get Started
            </Button>
          </Link>
          <Link to="/auth/login">
            <Button size="lg" variant="outline" className="h-11">
              Sign In
            </Button>
          </Link>
        </div>
      </section>

      {/* Features Section */}
      <section className="container mx-auto border-t py-8 md:py-24">
        <div className="mx-auto grid justify-center gap-4 sm:grid-cols-2 md:max-w-[64rem] md:grid-cols-3">
          <div className="relative overflow-hidden rounded-lg border bg-background p-6">
            <div className="flex flex-col gap-2">
              <h3 className="font-bold">API Key Management</h3>
              <p className="text-sm text-muted-foreground">
                Securely manage your API keys and access tokens for FHE
                operations.
              </p>
            </div>
          </div>

          <div className="relative overflow-hidden rounded-lg border bg-background p-6">
            <div className="flex flex-col gap-2">
              <h3 className="font-bold">Private Decryption</h3>
              <p className="text-sm text-muted-foreground">
                Request and manage private decryptions with our secure relayer
                network.
              </p>
            </div>
          </div>

          <div className="relative overflow-hidden rounded-lg border bg-background p-6">
            <div className="flex flex-col gap-2">
              <h3 className="font-bold">Developer Tools</h3>
              <p className="text-sm text-muted-foreground">
                Access comprehensive SDKs, documentation, and testing tools.
              </p>
            </div>
          </div>
        </div>
      </section>

      {/* Pricing Preview Section */}
      <section className="container mx-auto border-t py-12 md:py-24">
        <div className="mx-auto max-w-2xl text-center">
          <h2 className="text-3xl font-semibold sm:text-4xl">
            Plans for every stage
          </h2>
          <p className="mt-4 text-muted-foreground">
            Compare subscriptions tailored to building, testing, and scaling
            fully homomorphic encryption workloads.
          </p>
        </div>

        {error && (
          <div className="mx-auto mt-8 max-w-2xl rounded-md border border-destructive/50 bg-destructive/10 p-4 text-destructive">
            {error}
          </div>
        )}

        {plans.length === 0 && !error ? (
          <div className="mx-auto mt-12 max-w-2xl text-center text-muted-foreground">
            Subscription plans will appear here once they are configured in
            Moesif.
          </div>
        ) : (
          <div className="mt-12">
            <PlanCardGrid plans={plans.slice(0, 3)} />
          </div>
        )}

        <div className="mt-10 flex justify-center">
          <Link to="/pricing">
            <Button variant="outline">Explore full pricing</Button>
          </Link>
        </div>
      </section>
    </div>
  );
}
