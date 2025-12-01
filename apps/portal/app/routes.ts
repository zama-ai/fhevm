import {
  type RouteConfig,
  index,
  layout,
  route,
} from "@react-router/dev/routes";

export default [
  layout("layouts/main.tsx", [
    index("routes/home.tsx"),
    route("pricing", "features/checkout/routes/pricing.tsx"),
    route("auth/login", "features/auth/routes/login.tsx"),
    route("auth/signup", "features/auth/routes/signup.tsx"),
    route("auth/logout", "features/auth/routes/logout.tsx"),
    route("auth/callback", "features/auth/routes/callback.tsx"),
    layout("layouts/authenticated.tsx", [
      route("dashboard", "features/dashboard/routes/dashboard.tsx"),
      route("api-keys", "features/api-keys/routes/api-keys.tsx"),
      route("checkout", "features/checkout/routes/checkout.tsx"),
      route("return", "features/checkout/routes/return.tsx"),
    ]),
  ]),
] satisfies RouteConfig;
