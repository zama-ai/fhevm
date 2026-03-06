import { loadFixture } from "@nomicfoundation/hardhat-network-helpers";
import { expect } from "chai";
import hre from "hardhat";

import { KMSGeneration } from "../../typechain-types";
import { loadTestVariablesFixture } from "../utils";

describe("Key resharing tasks", function () {
  let kmsGeneration: KMSGeneration;

  before(async function () {
    const fixtureData = await loadFixture(loadTestVariablesFixture);
    kmsGeneration = fixtureData.kmsGeneration;
  });

  it("Should trigger PRSS init", async function () {
    await hre.run("task:prssInit", { useInternalGatewayConfigAddress: true });
    const filter = kmsGeneration.filters.PRSSInit();
    const events = await kmsGeneration.queryFilter(filter);
    expect(events.length).to.be.greaterThan(0);
  });
});
