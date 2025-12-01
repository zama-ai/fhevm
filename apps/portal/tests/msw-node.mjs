import { setupServer } from "msw/node";
import { handlers } from "./mocks/auth0-handlers.js";

const server = setupServer(...handlers);

server.listen({ onUnhandledRequest: "bypass" });
console.log("🔶 MSW node server listening (Auth0 mocks active)");

const shutdown = () => {
  server.close();
  console.log("🔶 MSW node server stopped");
};

process.once("SIGINT", shutdown);
process.once("SIGTERM", shutdown);
