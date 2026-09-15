import { expect, test } from "bun:test";
import {
  resolveSenderTransports,
  senderTransportForTag,
  senderTransportFromSource,
  senderSourceRevision,
  selectedSenderTags,
} from "./resolve/sender-transport";
import { testDefaultScenario } from "./test-fixtures";
import { applyVersionEnvOverrides, presetBundle } from "./resolve/target";

const wsSource = "ProviderBuilder::new().connect_ws(WsConnect::new(url)).await?";
const httpSource = "ProviderBuilder::new().connect_reqwest(gateway_http_client(url)?, url.clone())";

test("source transport distinguishes the old sender, current sender, and backport", () => {
  expect(senderTransportFromSource(wsSource)).toBe("ws");
  expect(senderTransportFromSource(httpSource)).toBe("http");
  expect(() => senderTransportFromSource("unknown API")).toThrow("Cannot determine");
  expect(() => senderTransportFromSource(wsSource + httpSource)).toThrow("Cannot determine");
});

test("resolution uses the selected sender override and resolves each pinned image independently", async () => {
  const bundle = applyVersionEnvOverrides(presetBundle("sha", "c2f416b", "sha.json"), {
    COPROCESSOR_TX_SENDER_VERSION: "b8406d0",
  });
  const reads: string[] = [];
  const resolved = await resolveSenderTransports(bundle, ["d5f946e", "b8406d0"], {
    readSource: async (tag) => {
      reads.push(tag);
      return tag === "b8406d0" ? wsSource : httpSource;
    },
  });
  expect(reads.sort()).toEqual(["b8406d0", "d5f946e"]);
  expect(senderTransportForTag(resolved, "b8406d0")).toBe("ws");
  expect(senderTransportForTag(resolved, "d5f946e")).toBe("http");
  expect(resolved.senderGatewayTransports?.c2f416b).toBeUndefined();
});

test("a serialized lock renders offline without source lookups and rejects a new unresolved pin", async () => {
  const bundle = presetBundle("sha", "b8406d0", "sha.json");
  const resolved = await resolveSenderTransports(bundle, undefined, { readSource: async () => wsSource });
  const saved = JSON.parse(JSON.stringify(resolved));
  const restored = await resolveSenderTransports(saved, undefined, {
    offline: true,
    readSource: async () => {
      throw Error("unexpected lookup");
    },
  });
  expect(senderTransportForTag(restored, "b8406d0")).toBe("ws");
  await expect(resolveSenderTransports(saved, ["new-image"], { offline: true })).rejects.toThrow(
    "fhevm-cli resolve --lock-file",
  );
  expect(() => senderTransportForTag(restored, "new-image")).toThrow("No resolved");
});

test("branch image tags identify their publishing revision", () => {
  const revision = "b8406d0a7ebd0cb3af308d674937fb3c1d7142a2";
  expect(senderSourceRevision(`feature-solana-${revision}`)).toBe(revision);
  expect(senderSourceRevision("v0.14.1")).toBe("v0.14.1");
});

test("local sender builds do not require registry metadata", async () => {
  const bundle = presetBundle("sha", "unused-image", "sha.json");
  const tags = selectedSenderTags(bundle, testDefaultScenario(), [{ group: "coprocessor" }]);
  expect(tags).toEqual([]);
  await expect(resolveSenderTransports(bundle, tags, { offline: true })).resolves.toMatchObject({
    senderGatewayTransports: {},
  });
});
