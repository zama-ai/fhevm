import { test as base, expect } from "@playwright/test";
import { createNetworkFixture, type NetworkFixture } from "@msw/playwright";
import { handlers } from "./mocks/auth0-handlers.js";

interface Fixtures {
  network: NetworkFixture;
}

export const test = base.extend<Fixtures>({
  network: createNetworkFixture({
    initialHandlers: handlers,
  }),
});

export { expect };
